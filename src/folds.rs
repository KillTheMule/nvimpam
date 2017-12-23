//! This module provides the [FoldList](folds/struct.FoldList.html) struct to
//! manage folds in a buffer.
//!
//! Example usage:
//!
//! ```
//! # use nvimpam_lib::folds::FoldList;
//! # use nvimpam_lib::card::keyword::Keyword;
//! let mut foldlist = FoldList::new();
//! foldlist.checked_insert(1,2, Keyword::Node).map_err(|e| println!("{}", e));
//! assert!(foldlist.remove(2,3).is_err());
//! assert!(foldlist.remove(1,2).is_ok());
//! ```
//!
use std::collections::BTreeMap;
use std::collections::btree_map::Entry;

use failure::Error;
use failure::Fail;
use failure::ResultExt;

use neovim_lib::{Neovim, NeovimApi};

use card::keyword::Keyword;

/// Holds the fold data of the buffer. A fold has the following data:
/// Linenumbers start, end (indexed from 1), and a
/// [Keyword](../cards/enum.Keyword.html).
pub struct FoldList {
  /// List of folds, keyed by [start, end], valued by Keyword, sorted
  /// lexicographically on [start, end].
  folds: BTreeMap<[u64; 2], Keyword>,
  /// List of folds, keyed by [end, start], valued by Keyword, sorted
  /// lexicographically on [end, start].  Kept synchronous to Folds by the
  /// struct methods.
  folds_inv: BTreeMap<[u64; 2], Keyword>,
}

impl FoldList {
  /// Create a new FoldList. There does not seem to
  /// be a way to create one with a predetermined capacity.
  pub fn new() -> FoldList {
    FoldList {
      folds: BTreeMap::new(),
      folds_inv: BTreeMap::new(),
    }
  }

  /// Clear FoldList, by clearing the BTreeMap's individually
  pub fn clear(&mut self) {
    self.folds.clear();
    self.folds_inv.clear();
  }

  /// Insert a fold (start, end) into the FoldList. Returns an error if that
  /// fold is already in the list. In that case, it needs to be
  /// [removed](struct.FoldList.html#method.remove) beforehand.
  fn insert(&mut self, start: u64, end: u64, kw: Keyword) -> Result<(), Error> {
    match self.folds.entry([start, end]) {
      Entry::Occupied(_) => Err(format_err!("Fold already in foldlist!")),
      Entry::Vacant(entry) => {
        entry.insert(kw.clone());
        self.folds_inv.insert([end, start], kw);
        Ok(())
      }
    }
  }

  /// Insert a fold (start, end) into the FoldList. If the length of the fold
  /// is less than 2, or the card is a Comment, we silently return without
  /// doing anything.  Otherwise, we call the internal insert function that
  /// returns an error if the fold is already in the list. In that case, it
  /// needs to be [removed](struct.FoldList.html#method.remove) beforehand.
  pub fn checked_insert(
    &mut self,
    start: u64,
    end: u64,
    kw: Keyword,
  ) -> Result<(), Error> {
    if start < end && kw != Keyword::Comment {
      self.insert(start, end, kw)?
    }
    Ok(())
  }

  /// Remove a fold (start, end) from the foldlist. Only checks if the fold
  /// is in the FoldList, and returns an error otherwise.
  pub fn remove(&mut self, start: u64, end: u64) -> Result<(), Error> {
    self.folds.remove(&[start, end]).ok_or(format_err!(
      "Could not remove fold from foldlist"
    ))?;
    self.folds_inv.remove(&[end, start]).ok_or(format_err!(
      "Could not remove fold from inverse foldlist!"
    ))?;

    Ok(())
  }

  /// Remove all the entries from the FoldList, and iterate over lines to
  /// populate it with new ones
  pub fn recreate_all(&mut self, lines: &[String]) -> Result<(), Error> {
    self.clear();
    self.add_folds(lines)
  }

  /// Delete all folds in nvim, and create the ones from the FoldList
  /// TODO: Check if we're using the best method to send
  pub fn resend_all(&self, nvim: &mut Neovim) -> Result<(), Error> {
    nvim.command("normal! zE").context("'normal! zE' failed")?;

    // TODO: use nvim_call_atomic
    for (range, _) in self.folds.iter() {
      nvim
        .command(&format!("{},{}fo", range[0] + 1, range[1] + 1))
        .with_context(|e| {
          e.clone().context(format!(
            "'{},{}fo' failed!",
            range[0] + 1,
            range[1] + 1
          ))
        })?;
    }

    Ok(())
  }

  /// Turn the FoldList into a Vec, containing the tuples (start, end,
  /// Keyword)
  pub fn into_vec(self) -> Vec<(u64, u64, Keyword)> {
    let mut v = Vec::new();
    for (s, card) in self.folds {
      let start = s[0];
      let end = s[1];
      v.push((start, end, card));
    }
    v
  }

  /// Parse an array of strings into a [FoldList](struct.FoldList.html). The
  /// foldlist is cleared as a first step.
  ///
  /// Comments are subsumed into a fold of a different type, if the
  /// surrounding folds are of the same type. This will create a larger fold
  /// containing the surrounding folds and the comments, and will be of the
  /// type of the surrounding folds. Otherwise, folds will form their own
  /// fold range.
  pub fn add_folds<T: AsRef<str>>(&mut self, lines: &[T]) -> Result<(), Error> {

    if lines.len() == 0 {
      return Err(format_err!("No lines passed!"));
    }

    let mut it = lines.iter().enumerate();
    let mut curkw = None;
    let mut curline;
    let mut curidx = 0;


    loop {
      // Advance to the next keyword line
      if curkw.is_none() {
        let tmp = it.find(|&(_, l)| Keyword::parse(l).is_some());
        match tmp {
          None => return Ok(()),
          Some((i, l)) => {
            curidx = i;
            curline = l;
            curkw = Keyword::parse(curline);
          }
        };
      };

      // skip comments
      if curkw == Some(Keyword::Comment) {
        let tmp =
          it.find(|&(_, l)| Keyword::parse(l) != Some(Keyword::Comment));
        match tmp {
          None => return Ok(()),
          Some((i, l)) => {
            curidx = i;
            curline = l;
            curkw = Keyword::parse(curline);
          }
        };
      }
      match curkw {
        None => continue,
        Some(Keyword::Comment) => unreachable!(),
        Some(c) => {
          let foldend = c.get_foldend(&mut it);

          if let Some(i) = foldend.0 {
            self.insert(curidx as u64, i, c)?;
          }
          curkw = foldend.1;

          if let Some(i) = foldend.2 {
            curidx = i as usize;
          }
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use card::keyword::Keyword;

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
  fn fold_end_01() {
    use card::keyword::Keyword::*;
    use folds::FoldList;

    let mut v = vec![(0, 3, Node), (7, 15, Shell), (18, 19, Node)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&LINES);
    assert_eq!(v, foldlist.into_vec());

    v = vec![(3, 11, Shell), (14, 15, Node)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&LINES[4..]);
    assert_eq!(v, foldlist.into_vec());

    v = vec![(1, 9, Shell), (12, 13, Node)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&LINES[6..]);
    assert_eq!(v, foldlist.into_vec());

    v = vec![(1, 2, Shell)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&LINES[13..19]);
    assert_eq!(v, foldlist.into_vec());
  }


  const CARD_MASS: [&'static str; 7] = [
    "$ NSMAS - Nonstructural mass",
    "$#       IDNODMS            MASS            MLEN            MARE            MVOL",
    "NSMAS /        1              0.                                                ",
    "$#                                                                         TITLE",
    "NAME NSMAS / ->1                                                                ",
    "        ELE",
    "        END",
  ];

  #[test]
  fn newfold_mass() {
    use card::keyword::Keyword::*;
    use folds::FoldList;

    let mut it = CARD_MASS.iter().enumerate();
    let _ = it.next();
    let _ = it.next();
    let _ = it.next();

    let (end, kw, idx) = Nsmas.get_foldend(&mut it);
    assert_eq!(end, Some(5));
    assert_eq!(kw, None);
    assert_eq!(idx, Some(6));

    let v = vec![(2, 5, Nsmas)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_MASS);

    assert_eq!(v, foldlist.into_vec());
  }

  #[test]
  fn newfold_lines() {
    use self::Keyword::*;
    use folds::FoldList;

    let v = vec![(0, 3, Node), (7, 15, Shell), (18, 19, Node)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&LINES);
    assert_eq!(v, foldlist.into_vec());
  }
}
