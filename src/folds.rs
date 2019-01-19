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
use std::collections::{btree_map::Entry, BTreeMap};

use failure::{self, Error, ResultExt};

use neovim_lib::{Neovim, NeovimApi, Value};

use itertools::Itertools;

use crate::card::keyword::Keyword;
use crate::highlights::HighlightGroup as Hl;
use crate::lines::{Line, ParsedLine};
use crate::nocommentiter::CommentLess;

macro_rules! unwrap_or_ok {
  ($option:expr) => {
    match $option {
      None => return Ok(()),
      Some(t) => t,
    }
  };
  ($option:expr, $ret:expr) => {
    match $option {
      None => return Ok($ret),
      Some(t) => t,
    }
  };
}

/// Holds the fold data of the buffer. A fold has the following data:
/// Linenumbers start, end (indexed from 0), and a
/// [Keyword](::card::Keyword).
#[derive(Default, Debug)]
pub struct FoldList {
  /// List of folds, keyed by [start, end], valued by
  /// `([Keyword](::card::keyword::Keyword), String)`, where the `String` is
  /// the fold's text. Sorted lexicographically on [start, end] (linenumbers
  /// starting at 0).
  folds: BTreeMap<[u64; 2], (Keyword, String)>,
  /// List of level 2 folds (i.e. containing level 1 folds), keyed by [start,
  /// end], valued by `([Keyword](::card::keyword::Keyword), String)`, where
  /// the `String` is the fold's text. Sorted lexicographically on [start,
  /// end] (linenumbers starting at 0).
  folds_level2: BTreeMap<[u64; 2], (Keyword, String)>,
  /// Highlights
  highlights_by_line: BTreeMap<(u64, u8, u8), Hl>,
}

impl FoldList {
  /// Create a new FoldList. There does not seem to
  /// be a way to create one with a predetermined capacity.
  pub fn new() -> FoldList {
    FoldList {
      folds: BTreeMap::new(),
      folds_level2: BTreeMap::new(),
      highlights_by_line: BTreeMap::new(),
    }
  }

  /// Clear FoldList, by clearing the BTreeMap's individually
  pub fn clear(&mut self) {
    self.folds.clear();
    self.folds_level2.clear();
    self.highlights_by_line.clear();
  }

  /// Insert a level 1 fold `([start, end], Keyword)` into the FoldList.
  /// Returns an error if that fold is already in the list. In that case,
  /// it needs to be [removed](struct.FoldList.html#method.remove) beforehand.
  fn insert(&mut self, start: u64, end: u64, kw: Keyword) -> Result<(), Error> {
    match self.folds.entry([start, end]) {
      Entry::Occupied(_) => {
        return Err(failure::err_msg("Fold already in foldlist!"))
      }
      Entry::Vacant(entry) => {
        // TODO: Maybe use a &'static str without #lines for cards with ownfold
        // = true?
        entry.insert((kw, format!(" {} lines: {:?} ", end - start + 1, kw)));
      }
    }
    Ok(())
  }

  // TODO: Pass newfolds by value
  pub fn splice(
    &mut self,
    newfolds: &mut FoldList,
    firstline: usize,
    lastline: usize,
    added: i64,
  ) {
    // Deal with highlights

    let first_to_delete = self
      .highlights_by_line
      .range((firstline as u64, 0, 0)..)
      .next()
      .map(|f| *(f.0));

    let mut to_change = match first_to_delete {
      Some(ftd) => self.highlights_by_line.split_off(&ftd),
      None => BTreeMap::new(),
    };

    let first_to_move = to_change
      .range((lastline as u64, 0, 0)..)
      .next()
      .map(|f| *(f.0));

    let to_move = match first_to_move {
      Some(ftm) => to_change.split_off(&ftm),
      None => BTreeMap::new(),
    };

    for (k, v) in newfolds.highlights_by_line.iter() {
      self.add_highlight(k.0 + firstline as u64, k.1, k.2, *v);
    }

    for (k, v) in to_move.iter() {
      self.add_highlight((k.0 as i64 + added) as u64, k.1, k.2, *v);
    }

    // Deal with folds

    let mut to_delete = vec![];
    let mut to_split = vec![];
    let mut last_before = None;
    let mut first_after = None;
    for (k, v) in self.folds.iter() {
      if (k[0] as usize) < firstline {
        last_before = Some((*k, v.0));
      }
      if lastline <= k[1] as usize && first_after.is_none() {
        first_after = Some((*k, v.0));
      }

      if firstline <= k[0] as usize && (k[0] as usize) < lastline {
        if (k[1] as usize) < lastline {
          to_delete.push(*k);
        } else {
          to_split.push((*k, v.0));
        }
      } else if ((firstline as usize) <= k[1] as usize)
        && ((k[1] as usize) < lastline)
      {
        // from the if above, we can assume k[0] < firstline
        to_split.push((*k, v.0));
      } else if (k[0] as usize) < firstline && lastline <= k[1] as usize {
        to_split.push((*k, v.0))
      }

      if lastline <= k[0] as usize {
        break;
      }
    }

    for k in to_delete {
      self.folds.remove(&k);
    }

    for (k, v) in to_split.into_iter() {
      self.folds.remove(&k);

      if k[0] < firstline as u64 {
        let _ = self.checked_insert(k[0], firstline as u64 - 1, v);
        last_before = Some(([k[0], firstline as u64 - 1], v))
      }

      if (lastline as u64) <= k[1] {
        let _ = self.checked_insert(lastline as u64, k[1], v);
        first_after = Some(([lastline as u64, k[1]], v));
      }
    }

    let first_new = match newfolds.folds.iter().next() {
      Some((k, v)) => Some((*k, v.0)),
      None => None,
    };
    let mut merge_to_first = None;
    let _ = last_before.map(|(k1, v1)| {
      first_new.map(|(_, v2)| {
        if v1 == v2 {
          self.folds.remove(&k1);
          merge_to_first = last_before;
        }
      })
    });

    let last_new = match newfolds.folds.range([0, 0]..).next_back() {
      Some((k, v)) => Some((*k, v.0)),
      None => None,
    };
    let mut merge_to_last = None;
    let _ = first_after.map(|(k1, v1)| {
      last_new.map(|(_, v2)| {
        if v1 == v2 {
          self.folds.remove(&k1);
          merge_to_last = first_after;
        }
      })
    });

    let first_fold_to_move =
      match self.folds.range([lastline as u64, 0]..).next() {
        Some((i, k)) => Some((*i, k.0)),
        None => None,
      };

    if let Some((f, _)) = first_fold_to_move {
      let to_move = self.folds.split_off(&f);

      for (k, v) in to_move.iter() {
        let _ = self.insert(
          (k[0] as i64 + added) as u64,
          (k[1] as i64 + added) as u64,
          v.0,
        );
      }
    }

    let mut last_added = None;
    for (k, v) in newfolds.folds.iter() {
      if let Some((k1, _)) = merge_to_first {
        let _ = self.insert(k1[0], k[1] + firstline as u64, v.0);
        last_added = Some([k1[0], k[1] + firstline as u64]);
        merge_to_first = None;
      } else {
        let _ =
          self.insert(k[0] + firstline as u64, k[1] + firstline as u64, v.0);
        last_added = Some([k[0] + firstline as u64, k[1] + firstline as u64]);
      }
    }

    if let Some(i) = last_added {
      if let Some((k2, v2)) = merge_to_last {
        self.folds.remove(&i);
        let _ = self.insert(i[0], (k2[1] as i64 + added) as u64, v2);
      }
    }

    // TODO: Should not need to call clear myself here
    let _ = self.recreate_level2();
  }

  pub fn add_highlight(&mut self, line: u64, start: u8, end: u8, typ: Hl) {
    match self.highlights_by_line.entry((line, start, end)) {
      Entry::Vacant(entry) => {
        entry.insert(typ);
      }
      Entry::Occupied(mut entry) => {
        *entry.get_mut() = typ;
      }
    }
  }

  pub fn extend_highlights<T>(&mut self, it: T)
  where
    T: IntoIterator<Item = ((u64, u8, u8), Hl)>,
  {
    self.highlights_by_line.extend(it)
  }

  /// Insert a level 1 fold `([start, end], Keyword)` into the FoldList. If
  /// `end < start`, we silently return.  Otherwise, we call the internal
  /// insert function that returns an error if the fold is already in the
  /// list. In that case, it needs to be
  /// [removed](struct.FoldList.html#method.remove)
  /// beforehand.
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

  /// Remove a level 1 fold [start, end] from the foldlist. Only checks if the
  /// fold is in the FoldList, and returns an error otherwise.
  pub fn remove(&mut self, start: u64, end: u64) -> Result<(), Error> {
    self
      .folds
      .remove(&[start, end])
      .ok_or_else(|| failure::err_msg("Could not remove fold from foldlist"))?;
    Ok(())
  }

  /// Remove all the entries from the FoldList, and iterate over lines to
  /// populate it with new ones. Then recreate the [level 2
  /// folds](::folds::FoldList::folds_level2).
  pub fn recreate_all(
    &mut self,
    keywords: &[Option<Keyword>],
    lines: &[Line],
  ) -> Result<(), Error> {
    self.clear();
    self.add_folds(keywords, lines)?;
    self.recreate_level2()
  }

  /// Recreate the level 2 folds from the level 1 folds. If there's no or one
  /// level 1 fold, `Ok(())` is returned.
  fn recreate_level2(&mut self) -> Result<(), Error> {
    self.folds_level2.clear();

    if self.folds.len() < 2 {
      return Ok(());
    }

    let grouped = self.folds.iter().group_by(|(_, &(kw, _))| kw);

    for (kw, mut group) in &grouped {
      let mut group = group.enumerate();
      let firstfold = group.next().expect("Empty group from group_by!").1;
      let (nr, lastfold) = match group.last() {
        None => continue, // only 1 fold in group
        Some((i, e)) => (i, e),
      };

      let firstline = firstfold.0[0];
      let lastline = lastfold.0[1];

      if firstline < lastline {
        match self.folds_level2.entry([firstline, lastline]) {
          Entry::Occupied(_) => {
            return Err(failure::err_msg("Fold already in foldlist_level2!"))
          }
          Entry::Vacant(entry) => {
            entry.insert((kw, format!(" {} {:?}s ", nr + 1, kw)));
          }
        }
      }
    }
    Ok(())
  }

  /// Delete all folds in nvim, and create the ones from the FoldList.
  pub fn resend_all(&self, nvim: &mut Neovim) -> Result<(), Error> {
    let luafn = "require('nvimpam').update_folds(...)";
    let mut luaargs = vec![];

    for (range, (_, text)) in self.folds.iter().chain(&self.folds_level2) {
      luaargs.push(Value::from(vec![
        Value::from(range[0] + 1),
        Value::from(range[1] + 1),
        Value::from(text.to_string()),
      ]));
    }

    nvim
      .execute_lua(luafn, vec![Value::from(luaargs)])
      .context("Execute lua failed")?;

    Ok(())
  }

  /// Highlight all the lines in the given region
  // TODO: efficient? correct?
  pub fn highlight_region(
    &self,
    nvim: &mut Neovim,
    firstline: u64,
    lastline: u64,
  ) -> Result<(), Error> {
    let curbuf = nvim.get_current_buf()?;
    let mut calls: Vec<Value> = vec![];

    calls.push(
      vec![
        Value::from("nvim_buf_clear_highlight".to_string()),
        vec![
          curbuf.get_value().clone(),
          Value::from(5),
          Value::from(firstline),
          Value::from(lastline),
        ]
        .into(),
      ]
      .into(),
    );

    for ((l, s, e), t) in self.highlights_by_line.iter() {
      if firstline <= *l && *l < lastline {
        let st: &'static str = (*t).into();
        calls.push(
          vec![
            Value::from("nvim_buf_add_highlight".to_string()),
            vec![
              curbuf.get_value().clone(),
              Value::from(5),
              Value::from(st.to_string()),
              Value::from(*l),
              Value::from(u64::from(*s)),
              Value::from(u64::from(*e)),
            ]
            .into(),
          ]
          .into(),
        );
      } else if *l > lastline {
        break;
      }
    }
    nvim.call_atomic(calls).context("call_atomic failed")?;
    Ok(())
  }

  /// Copy the elements of a FoldList of the given level into a Vec, containing
  /// the tuples (start, end, Keyword)
  pub fn to_vec(&self, level: u8) -> Vec<(u64, u64, Keyword)> {
    if level == 1 {
      self.folds.iter().map(|(r, (k, _))| (r[0], r[1], *k)).collect()
    } else if level == 2 {
      self
        .folds_level2
        .iter()
        .map(|(r, (k, _))| (r[0], r[1], *k))
        .collect()
    } else {
      unimplemented!()
    }
  }

  /// Parse an array of `Option<Keyword>`s into a
  /// [`FoldList`](::folds::FoldList). The foldlist is cleared as a first step.
  ///
  /// Creates only level 1 folds. Depending on the
  /// [`ownfold`](::card::Card::ownfold) parameter in the
  /// definition of the card in the [carddata](::carddata) module, each card
  /// will be in an own fold, or several adjacent (modulo comments) cards will
  /// be subsumed into a fold.
  pub fn add_folds(
    &mut self,
    keywords: &[Option<Keyword>],
    lines: &[Line],
  ) -> Result<(), Error> {
    debug_assert!(keywords.len() == lines.len());
    let mut li = keywords
      .iter()
      .zip(lines)
      .enumerate()
      .map(ParsedLine::from)
      .remove_comments();

    let mut foldstart;
    let mut foldend;
    let mut foldkw;
    let mut skipped;

    let mut nextline = unwrap_or_ok!(li.skip_to_next_keyword());

    loop {
      foldkw = nextline.keyword;
      foldstart = nextline.number;
      skipped = li.skip_fold(&nextline, self);

      // The latter only happens when a file ends after the only line of a card
      foldend = skipped.skip_end.unwrap_or_else(|| lines.len() - 1);

      self.checked_insert(foldstart as u64, foldend as u64, *foldkw)?;

      if let Some(Some(kl)) =
        skipped.nextline.map(|pl| pl.try_into_keywordline())
      {
        nextline = kl;
      } else {
        nextline = unwrap_or_ok!(li.skip_to_next_keyword());
      }
    }
  }
}

#[cfg(test)]
macro_rules! splicetest {
  ($fn: ident; existing: $([$($e: expr),+]),+; new: $([$($f: expr),+]),+;
  $first: expr, $last: expr, $added: expr; expected: $([$($g: expr),+]),+ ) => {
    #[test]
    fn $fn() {
      use crate::folds::FoldList;
      use crate::card::keyword::Keyword::*;

      let mut oldfolds = FoldList::new();
      $(let _ = oldfolds.insert($($e),+);)+

      let mut newfolds = FoldList::new();
      $(let _ = newfolds.insert($($f),+);)+

      oldfolds.splice(&mut newfolds, $first, $last, $added);
      let v = vec![$( ($($g),+ ),)+];

      assert_eq!(v, oldfolds.to_vec(1));
    }
  };
}

#[cfg(test)]
mod tests {
  use crate::card::keyword::Keyword::*;

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

  cardtest!(
    fold_general1,
    LINES,
    vec![(0, 3, Node), (5, 5, Shell), (7, 15, Shell), (18, 19, Node)]
  );

  cardtest!(
    fold_general2,
    LINES[4..],
    vec![(1, 1, Shell), (3, 11, Shell), (14, 15, Node)]
  );

  cardtest!(
    fold_general3,
    LINES[6..],
    vec![(1, 9, Shell), (12, 13, Node)]
  );

  cardtest!(
    fold_general4,
    LINES[13..19],
    vec![(1, 2, Shell), (5, 5, Node)]
  );

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

  cardtest!(
    fold_general_gather,
    LINES2,
    vec![
      (0, 3, Node),
      (5, 5, Shell),
      (6, 7, Node),
      (10, 14, Shell),
      (17, 19, Node),
      (20, 23, Shell),
    ]
  );

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

  cardtest!(
    fold_level2_rbodies,
    RBODIES,
    vec![
      (0, 2, Rbody0),
      (3, 5, Rbody0),
      (6, 9, Rbody0),
      (10, 12, Rbody0),
    ],
    vec![(0, 12, Rbody0)]
  );

  splicetest!(splice_folds_trivial;
    existing: [0, 4, Node], [10, 14, PartSolid];
    new: [0, 6, Shell];
    7, 9, 5;
    expected: [0, 4, Node],[7, 13, Shell],[15, 19, PartSolid]
  );

  splicetest!(splice_folds_add_below;
    existing: [0, 4, Node], [7, 9, Shell], [15, 19, PartSolid];
    new: [0, 1, Node];
    5, 6, 1;
    expected: [0, 6, Node], [8, 10, Shell], [16, 20, PartSolid]
  );

  splicetest!(splice_folds_add_after;
    existing: [0, 6, Node], [8, 10, Shell], [16, 20, PartSolid];
    new: [0, 1, Shell];
    8, 8, 2;
    expected: [0, 6, Node], [8, 12, Shell], [18, 22, PartSolid]
  );

  splicetest!(splice_folds_inbetween;
    existing: [0, 6, Node], [8, 10, Shell];
    new: [0, 3, Node];
    2, 2, 4;
    expected: [0, 10, Node], [12, 14, Shell]
  );

  splicetest!(splice_folds_cut_upper;
    existing: [0, 6, Node];
    new: [0, 1, Shell];
    3, 7, -2;
    expected: [0, 2, Node], [3, 4, Shell]
  );

  splicetest!(splice_folds_cut_lower;
    existing: [0, 6, Node];
    new: [0, 1, Shell];
    0, 4, -2;
    expected: [0, 1, Shell], [2, 4, Node]
  );

  splicetest!(splice_folds_divide;
    existing: [0, 10, Node];
    new: [0,2, Shell];
    3, 3, 3;
    expected: [0, 2, Node], [3, 5, Shell], [6, 13, Node]
  );

}
