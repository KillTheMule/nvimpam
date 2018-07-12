//! This module provides the [`FoldList`](::folds::FoldList) struct to
//! manage folds in a buffer. It carries both level 1 folds as well as level 2
//! folds (i.e. folds that contain folds of level 1). All functions that
//! insert/remove/modify folds operate on level 1 folds, the only thing to be
//! done for the level 2 folds is regenerating them in full from the level 1
//! folds.
//!
//! Example usage:
//!
//! ```
//! # use nvimpam_lib::folds::FoldList;
//! # use nvimpam_lib::card::keyword::Keyword;
//! let mut foldlist = FoldList::new();
//! foldlist
//!   .checked_insert(1, 2, Keyword::Node)
//!   .map_err(|e| println!("{}", e));
//! assert!(foldlist.remove(2, 3).is_err());
//! assert!(foldlist.remove(1, 2).is_ok());
//! ```
//!
use std::collections::btree_map::Entry;
use std::collections::BTreeMap;

use failure;
use failure::Error;
use failure::ResultExt;

use neovim_lib::{Neovim, NeovimApi, Value};

use itertools::Itertools;

use card::keyword::Keyword;
use nocommentiter::CommentLess;
use skipresult::{SkipLine, SkipResult};

/// Holds the fold data of the buffer. A fold has the following data:
/// Linenumbers start, end (indexed from 1), and a
/// [Keyword](::card::Keyword).
#[derive(Default, Debug)]
pub struct FoldList {
  /// List of folds, keyed by [start, end], valued by Keyword, sorted
  /// lexicographically on [start, end].
  folds: BTreeMap<[u64; 2], Keyword>,
  /// List of level 2 folds (i.e. containing level 1 folds), keyed by [start,
  /// end], valued by Keyword, sorted lexicographically on [start, end].
  folds_level2: BTreeMap<[u64; 2], Keyword>,
  /// List of Strings to show as foldtext
  fold_texts: BTreeMap<[u64; 2], String>,
}

impl FoldList {
  /// Create a new FoldList. There does not seem to
  /// be a way to create one with a predetermined capacity.
  pub fn new() -> FoldList {
    FoldList {
      folds: BTreeMap::new(),
      folds_level2: BTreeMap::new(),
      fold_texts: BTreeMap::new(),
    }
  }

  /// Clear FoldList, by clearing the BTreeMap's individually
  pub fn clear(&mut self) {
    self.folds.clear();
    self.folds_level2.clear();
    self.fold_texts.clear();
  }

  /// Insert a level 1 fold (start, end) into the FoldList. Returns an error if
  /// that fold is already in the list. In that case, it needs to be
  /// [removed](struct.FoldList.html#method.remove) beforehand.
  fn insert(&mut self, start: u64, end: u64, kw: Keyword) -> Result<(), Error> {
    match self.folds.entry([start, end]) {
      Entry::Occupied(_) => {
        return Err(failure::err_msg("Fold already in foldlist!"))
      }
      Entry::Vacant(entry) => {
        entry.insert(kw);
      }
    }
    match self.fold_texts.entry([start, end]) {
      Entry::Occupied(_) => {
        return Err(failure::err_msg("Foldtext already in fold_texts!"))
      }
      Entry::Vacant(entry) => {
        entry.insert(format!(" {} lines: {:?} ", end - start + 1, kw));
      }
    }
    Ok(())
  }

  /// Insert a level 1 fold (start, end) into the FoldList. If `end < start`,
  /// we silently return.  Otherwise, we call the internal insert function that
  /// returns an error if the fold is already in the list. In that case, it
  /// needs to be [removed](struct.FoldList.html#method.remove) beforehand.
  pub fn checked_insert(
    &mut self,
    start: u64,
    end: u64,
    kw: Keyword,
  ) -> Result<(), Error> {
    if start <= end {
      self.insert(start, end, kw)?
    }
    Ok(())
  }

  /// Remove a level 1 fold (start, end) from the foldlist. Only checks if the
  /// fold is in the FoldList, and returns an error otherwise.
  pub fn remove(&mut self, start: u64, end: u64) -> Result<(), Error> {
    self
      .folds
      .remove(&[start, end])
      .ok_or_else(|| failure::err_msg("Could not remove fold from foldlist"))?;
    self
      .fold_texts
      .remove(&[start, end])
      .ok_or_else(|| failure::err_msg("Could not remove fold from foldlist"))?;
    Ok(())
  }

  /// Remove all the entries from the FoldList, and iterate over lines to
  /// populate it with new ones
  pub fn recreate_all<T: AsRef<str>>(
    &mut self,
    lines: &[T],
  ) -> Result<(), Error> {
    self.clear();
    self.add_folds(lines)?;
    self.recreate_level2()
  }

  /// Recreate the level 2 folds from the level 1 folds. If there's no or one
  /// level 1 fold, `Ok(())` is returned.
  fn recreate_level2(&mut self) -> Result<(), Error> {
    self.folds_level2.clear();

    if self.folds.len() < 2 {
      return Ok(());
    }

    let grouped = self.folds.iter().group_by(|(_, &kw)| kw);

    for (kw, mut group) in &grouped {
      let first = group.next().expect("Empty group from group_by!");
      let mut last = None;
      let mut nr = 1;
      for x in group {
        last = Some(x);
        nr += 1;
      }
      let last = match last {
        None => continue, // only 1 fold in group
        Some(e) => e,
      };

      let firstline = first.0[0];
      let lastline = last.0[1];

      if firstline < lastline {
        match self.folds_level2.entry([firstline, lastline]) {
          Entry::Occupied(_) => {
            return Err(failure::err_msg("Fold already in foldlist_level2!"))
          }
          Entry::Vacant(entry) => {
            entry.insert(kw);
          }
        }
        match self.fold_texts.entry([firstline, lastline]) {
          Entry::Occupied(_) => {
            return Err(failure::err_msg("Foldtext already in fold_texts!"))
          }
          Entry::Vacant(entry) => {
            entry.insert(format!(" {} {:?}s ", nr, kw));
          }
        }
      }
    }
    Ok(())
  }

  /// Delete all folds in nvim, and create the ones from the FoldList
  /// https://github.com/KillTheMule/KillTheMule.github.io/blob/master/benchmark_rpc.md
  pub fn resend_all(&self, nvim: &mut Neovim) -> Result<(), Error> {
    let luafn = "require('nvimpam').update_foldtexts(...)";
    let mut luaargs = vec![];

    for (range, text) in &self.fold_texts {
      luaargs.push(Value::from(vec![
        Value::from(range[0] + 1),
        Value::from(range[1] + 1),
        Value::from(text.to_string()),
      ]));
    }
    nvim.execute_lua(luafn, vec![Value::from(luaargs)])?;

    // Just an estimate, not worth a lot
    let mut command = String::with_capacity(10 + 12 * self.folds.len());

    if !self.folds.is_empty() {
      for range in self.folds.keys() {
        let start = range[0];
        let end = range[1];
        if start < end {
          command.push_str(&format!("|{},{}fo", start + 1, end + 1));
        }
      }
      for range in self.folds_level2.keys() {
        command.push_str(&format!("|{},{}fo", range[0] + 1, range[1] + 1));
      }
      nvim.command("normal! zE")?;
      nvim.command(&command).context("Fold command failed!")?;
    }

    Ok(())
  }

  /// Turn the FoldList into a Vec, containing the tuples (start, end,
  /// Keyword)
  pub fn into_vec(self, level: u8) -> Vec<(u64, u64, Keyword)> {
    let mut v = Vec::new();
    if level == 1 {
      for (s, card) in self.folds {
        let start = s[0];
        let end = s[1];
        v.push((start, end, card));
      }
    } else if level == 2 {
      for (s, card) in self.folds_level2 {
        let start = s[0];
        let end = s[1];
        v.push((start, end, card));
      }
    }
    v
  }

  /// Parse an array of strings into a [FoldList](struct.FoldList.html). The
  /// foldlist is cleared as a first step.
  ///
  /// Creates only level 1 folds. Depending on the
  /// [ownfold](../card/struct.Card.html#structfield.ownfold) parameter in the
  /// definition of the card in the [carddata](::carddata) module, each card
  /// will be in an own fold, or several adjacent (modulo comments) cards will
  /// be subsumed into a fold.
  pub fn add_folds<T: AsRef<str>>(&mut self, lines: &[T]) -> Result<(), Error> {
    let mut li = lines.iter().enumerate().remove_comments();

    let mut foldstart;
    let mut foldend;
    let mut foldkw;
    let mut skipped;

    let mut nextline = match li.skip_to_next_keyword() {
      None => return Ok(()),
      Some(sl) => sl,
    };

    loop {
      foldkw = nextline.nextline_kw;
      foldstart = nextline.nextline.0;

      skipped = li.skip_fold(&nextline);

      if let Some(j) = skipped.skip_end {
        foldend = j;
      } else {
        // This only happens if the file ends directly after a GES
        foldend = lines.len() - 1;
      }
      self.checked_insert(foldstart as u64, foldend as u64, foldkw)?;

      if let SkipResult {
        nextline: Some(l),
        nextline_kw: Some(k),
        ..
      } = skipped
      {
        nextline = SkipLine {
          nextline: l,
          nextline_kw: k,
        }
      } else {
        nextline = match li.skip_to_next_keyword() {
          None => return Ok(()),
          Some(sl) => sl,
        };
      }
    }
  }
}

#[cfg(test)]
mod tests {

  const LINES: [&'static str; 20] = [
    /* 0 */
    "NODE  /        1              0.             0.5              0.",
    /* 1 */
    "NODE  /        1              0.             0.5              0.",
    /* 2 */
    "NODE  /        1              0.             0.5              0.",
    /* 3 */
    "NODE  /        1              0.             0.5              0.",
    /* 4 */
    "#Comment here",
    /* 5 */
    "SHELL /     3129       1       1    2967    2971    2970",
    /* 6 */
    "invalid line here",
    /* 7 */
    "SHELL /     3129       1       1    2967    2971    2970",
    /* 8 */
    "SHELL /     3129       1       1    2967    2971    2970",
    /* 9 */
    "#Comment",
    /* 10 */
    "#Comment",
    /* 11 */
    "SHELL /     3129       1       1    2967    2971    2970",
    /* 12 */
    "SHELL /     3129       1       1    2967    2971    2970",
    /* 13 */
    "$Comment",
    /* 14 */
    "SHELL /     3129       1       1    2967    2971    2970",
    /* 15 */
    "SHELL /     3129       1       1    2967    2971    2970",
    /* 16 */
    "$Comment",
    /* 17 */
    "#Comment",
    /* 18 */
    "NODE  /        1              0.             0.5              0.",
    /* 19 */
    "NODE  /        1              0.             0.5              0.",
  ];

  #[test]
  fn fold_general() {
    use card::keyword::Keyword::*;
    use folds::FoldList;

    let mut v =
      vec![(0, 3, Node), (5, 5, Shell), (7, 15, Shell), (18, 19, Node)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&LINES);
    assert_eq!(v, foldlist.into_vec(1));

    v = vec![(1, 1, Shell), (3, 11, Shell), (14, 15, Node)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&LINES[4..]);
    assert_eq!(v, foldlist.into_vec(1));

    v = vec![(1, 9, Shell), (12, 13, Node)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&LINES[6..]);
    assert_eq!(v, foldlist.into_vec(1));

    v = vec![(1, 2, Shell), (5, 5, Node)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&LINES[13..19]);
    assert_eq!(v, foldlist.into_vec(1));
  }

  const LINES2: [&'static str; 24] = [
    // 0
    "NODE  /        1              0.             0.5              0.",
    // 1
    "NODE  /        1              0.             0.5              0.",
    // 2
    "NODE  /        1              0.             0.5              0.",
    // 3
    "NODE  /        1              0.             0.5              0.",
    // 4
    "#Comment here",
    // 5
    "SHELL /     3129       1       1    2967    2971    2970",
    // 6
    "NODE  /     3129       1       1    2967    2971    2970",
    // 7
    "NODE  /     3129       1       1    2967    2971    2970",
    // 8
    "#Comment",
    // 9
    "#Comment",
    // 10
    "SHELL /     3129       1       1    2967    2971    2970",
    // 11
    "SHELL /     3129       1       1    2967    2971    2970",
    // 12
    "$Comment",
    // 13
    "SHELL /     3129       1       1    2967    2971    2970",
    // 14
    "SHELL /     3129       1       1    2967    2971    2970",
    // 15
    "$Comment",
    // 16
    "#Comment",
    // 17
    "NODE  /        1              0.             0.5              0.",
    // 18
    "NODE  /        1              0.             0.5              0.",
    // 19
    "NODE  /        1              0.             0.5              0.",
    // 20
    "SHELL /     3129       1       1    2967    2971    2970",
    // 21
    "SHELL /     3129       1       1    2967    2971    2970",
    // 22
    "SHELL /     3129       1       1    2967    2971    2970",
    // 23
    "SHELL /     3129       1       1    2967    2971    2970",
  ];

  #[test]
  fn fold_general_gather() {
    use card::keyword::Keyword::*;
    use folds::FoldList;

    let v = vec![
      (0, 3, Node),
      (5, 5, Shell),
      (6, 7, Node),
      (10, 14, Shell),
      (17, 19, Node),
      (20, 23, Shell),
    ];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&LINES2);
    assert_eq!(v, foldlist.into_vec(1));
  }

  const RBODIES: [&'static str; 13] = [
    "RBODY /        1               0       0                       0       0        ",
    "NAME RBODY / ->1                                                                ",
    "        END",
    "RBODY /        1               0       0                       0       0        ",
    "NAME RBODY / ->1                                                                ",
    "        END",
    "RBODY /        1               0       0                       0       0        ",
    "#COMMENT",
    "NAME RBODY / ->1                                                                ",
    "        END",
    "RBODY /        1               0       0                       0       0        ",
    "NAME RBODY / ->1                                                                ",
    "        END",
  ];

  #[test]
  fn fold_level2_rbodies() {
    use card::keyword::Keyword::*;
    use folds::FoldList;

    let v1 = vec![
      (0, 2, Rbody0),
      (3, 5, Rbody0),
      (6, 9, Rbody0),
      (10, 12, Rbody0),
    ];
    let v2 = vec![(0, 12, Rbody0)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.recreate_all(&RBODIES);
    assert_eq!(v1, foldlist.into_vec(1));
    foldlist = FoldList::new();
    let _ = foldlist.recreate_all(&RBODIES);
    assert_eq!(v2, foldlist.into_vec(2));
  }

}
