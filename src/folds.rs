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
    debug!(
      "Veclen: {}, last line: {{ {} }}",
      lines.len(),
      lines[lines.len() - 1]
    );
    Card::create_card_data(lines, self)?;

    Ok(())

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
}
