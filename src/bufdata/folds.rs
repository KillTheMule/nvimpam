//! Holds the `Folds` datastructure for the fold data associated with a buffer
use std::collections::{btree_map::Entry, BTreeMap};

use failure::Error;
use itertools::Itertools;

use crate::card::keyword::Keyword;

#[derive(Default, Debug)]
pub struct Folds(BTreeMap<[u64;2], (Keyword, String)>);

impl Folds {
  pub fn new() -> Self {
    Folds(BTreeMap::new())
  }

  pub fn clear(&mut self) {
    self.0.clear()
  }

  pub fn iter(&self) -> impl Iterator<Item = (&[u64;2], &(Keyword, String))> {
    self.0.iter()
  }

  pub fn len(&self) -> usize {
    self.0.len()
  }

  /// Insert a level 1 fold `([start, end], Keyword)` into the FoldList.
  /// Returns an error if that fold is already in the list. In that case,
  /// it needs to be [removed](struct.FoldList.html#method.remove) beforehand.
  fn insert(&mut self, start: u64, end: u64, kw: Keyword) -> Result<(), Error> {
    match self.0.entry([start, end]) {
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
      .0
      .remove(&[start, end])
      .ok_or_else(|| failure::err_msg("Could not remove fold from foldlist"))?;
    Ok(())
  }

  /// Copy the elements of a FoldList of the given level into a Vec, containing
  /// the tuples (start, end, Keyword)
  pub fn to_vec(&self) -> Vec<(u64, u64, Keyword)> {
    self.0.iter().map(|(r, (k, _))| (r[0], r[1], *k)).collect()
  }


  /// Recreate the level 2 folds from the level 1 folds. If there's no or one
  /// level 1 fold, `Ok(())` is returned.
  pub fn recreate_level2(&mut self, folds: &Folds) -> Result<(), Error> {
    self.0.clear();

    if folds.len() < 2 {
      return Ok(());
    }

    let grouped = folds.iter().group_by(|(_, &(kw, _))| kw);

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
        match self.0.entry([firstline, lastline]) {
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

  // TODO: Pass newfolds by value
  pub fn splice(
    &mut self,
    newfolds: &mut Folds,
    firstline: usize,
    lastline: usize,
    added: i64,
  ) {
    let mut to_delete = vec![];
    let mut to_split = vec![];
    let mut last_before = None;
    let mut first_after = None;
    for (k, v) in self.0.iter() {
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
      self.0.remove(&k);
    }

    for (k, v) in to_split.into_iter() {
      self.0.remove(&k);

      if k[0] < firstline as u64 {
        let _ = self.checked_insert(k[0], firstline as u64 - 1, v);
        last_before = Some(([k[0], firstline as u64 - 1], v))
      }

      if (lastline as u64) <= k[1] {
        let _ = self.checked_insert(lastline as u64, k[1], v);
        first_after = Some(([lastline as u64, k[1]], v));
      }
    }

    let first_new = match newfolds.0.iter().next() {
      Some((k, v)) => Some((*k, v.0)),
      None => None,
    };
    let mut merge_to_first = None;
    let _ = last_before.map(|(k1, v1)| {
      first_new.map(|(_, v2)| {
        if v1 == v2 {
          self.0.remove(&k1);
          merge_to_first = last_before;
        }
      })
    });

    let last_new = match newfolds.0.range([0, 0]..).next_back() {
      Some((k, v)) => Some((*k, v.0)),
      None => None,
    };
    let mut merge_to_last = None;
    let _ = first_after.map(|(k1, v1)| {
      last_new.map(|(_, v2)| {
        if v1 == v2 {
          self.0.remove(&k1);
          merge_to_last = first_after;
        }
      })
    });

    let first_fold_to_move =
      match self.0.range([lastline as u64, 0]..).next() {
        Some((i, k)) => Some((*i, k.0)),
        None => None,
      };

    if let Some((f, _)) = first_fold_to_move {
      let to_move = self.0.split_off(&f);

      for (k, v) in to_move.iter() {
        let _ = self.insert(
          (k[0] as i64 + added) as u64,
          (k[1] as i64 + added) as u64,
          v.0,
        );
      }
    }

    let mut last_added = None;
    for (k, v) in newfolds.0.iter() {
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
        self.0.remove(&i);
        let _ = self.insert(i[0], (k2[1] as i64 + added) as u64, v2);
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
      use crate::bufdata::folds::Folds;
      use crate::card::keyword::Keyword::*;

      let mut oldfolds = Folds::new();
      $(let _ = oldfolds.insert($($e),+);)+

      let mut newfolds = Folds::new();
      $(let _ = newfolds.insert($($f),+);)+

      oldfolds.splice(&mut newfolds, $first, $last, $added);
      let v = vec![$( ($($g),+ ),)+];

      assert_eq!(v, oldfolds.to_vec());
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
