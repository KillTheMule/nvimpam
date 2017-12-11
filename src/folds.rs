//! This module provides the [FoldList](folds/struct.FoldList.html) struct to
//! manage folds in a buffer.
//!
//! Example usage:
//!
//! ```
//! # use nvimpam_lib::folds::FoldList;
//! # use nvimpam_lib::cards::Card;
//! let mut foldlist = FoldList::new();
//! foldlist.insert(1,2, Card::Node).map_err(|e| println!("{}", e));
//! assert!(foldlist.remove(2,3).is_err());
//! assert!(foldlist.remove(1,2).is_ok());
//! ```
//!
use std::collections::BTreeMap;
use std::collections::btree_map::Entry;
use std::io::{Error, ErrorKind};

use neovim_lib::{Neovim, NeovimApi};

use cards::Card;

/// Holds the fold data of the buffer. A fold has the following data:
/// Linenumbers start, end (indexed from 1), and a
/// [Card](../cards/enum.Card.html).
pub struct FoldList {
  /// List of folds, keyed by [start, end], valued by Card, sorted
  /// lexicographically on [start, end].
  folds: BTreeMap<[u64; 2], Card>,
  /// List of folds, keyed by [end, start], valued by Card, sorted
  /// lexicographically on [end, start].  Kept synchronous to Folds by the
  /// struct methods.
  folds_inv: BTreeMap<[u64; 2], Card>,
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

  /// Insert a fold (start, end) into the FoldList. Returns an error if start
  /// is greater than end, or if that fold is already in the list. In that
  /// case, you need to remove it by calling FoldList.remove(start, end)
  /// beforehand.
  pub fn insert(
    &mut self,
    start: u64,
    end: u64,
    card: Card,
  ) -> Result<(), Box<Error>> {

    if start > end {
      return Err(Box::new(
        Error::new(ErrorKind::Other, "Folds need start <= end!"),
      ));
    }
    match self.folds.entry([start, end]) {
      Entry::Occupied(_) => Err(Box::new(Error::new(
        ErrorKind::Other,
        "Fold already in list!",
      ))),
      Entry::Vacant(entry) => {
        entry.insert(card.clone());
        self.folds_inv.insert([end, start], card);
        Ok(())
      }
    }
  }

  /// Remove a fold (start, end) from the foldlist. Only checks if the fold
  /// is in the FoldList, and returns an error otherwise.
  pub fn remove(&mut self, start: u64, end: u64) -> Result<(), Box<Error>> {

    if self.folds.remove(&[start, end]).is_none() {
      return Err(Box::new(Error::new(
        ErrorKind::Other,
        "Could not remove fold from foldlist",
      )));
    } else {
    }

    if self.folds_inv.remove(&[end, start]).is_none() {
      return Err(Box::new(Error::new(
        ErrorKind::Other,
        "Could not remove fold from foldlist",
      )));
    } else {
    }

    Ok(())
  }

  /// Remove all the entries from the FoldList, and iterate over lines to
  /// populate it with new ones
  pub fn recreate_all(&mut self, lines: &[String]) -> Result<(), Box<Error>> {
    self.clear();
    self.add_card_data(lines)
  }

  /// Delete all folds in nvim, and create the ones from the FoldList
  /// TODO: Check if we're using the best method to send
  pub fn resend_all(&self, nvim: &mut Neovim) -> Result<(), Box<Error>> {
    nvim.command("normal! zE").unwrap();

    let mut args: Vec<String> = Vec::new();

    for (range, _) in self.folds.iter() {
      args.push(format!("{},{}fo", range[0], range[1]));
    }
    debug!("Folds: {}", args.len());

    // TODO: use nvim_call_atomic
    for arg in args {
      nvim.command(&arg).unwrap();
    }

    Ok(())
  }

  /// Turn the FoldList into a Vec, containing the tuples (start, end, Card)
  pub fn into_vec(self) -> Vec<(u64, u64, Card)> {
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
  #[inline]
  pub fn add_card_data<T: AsRef<str>>(
    &mut self,
    lines: &[T],
  ) -> Result<(), Box<Error>> {
    let it = lines
      .iter()
      .map(|s| Card::parse_str(s.as_ref()))
      .enumerate();
    let mut curcardstart = 0;
    let mut curcard: Option<Card> = None;

    let mut last_before_comment = 0;

    for (i, linecard) in it {
      match linecard {
        None => {
          if i > 0 {
            if let Some(c) = curcard {
              if last_before_comment > 0 {
                self.insert(
                  curcardstart as u64,
                  last_before_comment as u64,
                  c,
                )?;
                if i - last_before_comment > 1 {
                  self.insert(
                    last_before_comment as u64 + 1,
                    i as u64 - 1,
                    Card::Comment,
                  )?;
                }
                last_before_comment = 0;
              } else {
                self.insert(curcardstart as u64, i as u64 - 1, c)?;
              }
            }
          }
          curcard = None;
          curcardstart = i;
        }
        Some(ref c) => {
          if linecard == curcard {
            last_before_comment = 0;
            continue;
          } else {
            if linecard == Some(Card::Comment) {
              if i > 1 && last_before_comment == 0 {
                last_before_comment = i - 1;
                continue;
              } else {
                if i == 0 {
                  curcard = Some(Card::Comment);
                  curcardstart = 0;
                }
              }
            } else {
              // linecard != curcard, and linecard != Some(Comment)
              if let Some(c) = curcard {
                if last_before_comment > 0 {
                  self.insert(
                    curcardstart as u64,
                    last_before_comment as u64,
                    c,
                  )?;
                  // probably redundant
                  if i > 0 {
                    self.insert(
                      last_before_comment as u64 + 1,
                      i as u64 - 1,
                      Card::Comment,
                    )?;
                  }
                  last_before_comment = 0;
                } else {
                  if i > 0 {
                    self.insert(curcardstart as u64, i as u64 - 1, c)?;
                  }
                }
              }
              curcard = Some(*c);
              curcardstart = i;
            }

          }
        }
      }
    }
    // When through the whole vec, need to insert a last card
    if let Some(c) = curcard {
      self.insert(curcardstart as u64, lines.len() as u64 - 1, c)?;
    }
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use cards::Card;

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
    use self::Card::*;
    use folds::FoldList;

    let mut v = vec![
      (0, 3, Node),
      (4, 4, Comment),
      (5, 5, Shell),
      (7, 15, Shell),
      (16, 17, Comment),
      (18, 19, Node),
    ];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_card_data(&LINES);
    assert_eq!(v, foldlist.into_vec());

    v = vec![
      (0, 0, Comment),
      (1, 1, Shell),
      (3, 11, Shell),
      (12, 13, Comment),
      (14, 15, Node),
    ];
    let mut foldlist = FoldList::new();
    foldlist.clear();
    let _ = foldlist.add_card_data(&LINES[4..]);
    assert_eq!(v, foldlist.into_vec());

    v = vec![(1, 9, Shell), (10, 11, Comment), (12, 13, Node)];
    let mut foldlist = FoldList::new();
    foldlist.clear();
    let _ = foldlist.add_card_data(&LINES[6..]);
    assert_eq!(v, foldlist.into_vec());
  }
}
