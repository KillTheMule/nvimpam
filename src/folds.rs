//! This module provides the [FoldList](struct.FoldList.html) struct to manage
//! folds in a buffer.
//!
//! Example usage:
//!
//! ```
//! # use nvimpam_lib::folds::{FoldList, CardType};
//! let mut foldlist = FoldList::new();
//! foldlist.insert(1,2, CardType::Node).map_err(|e| println!("{}", e));
//! assert!(foldlist.remove(2,3).is_err());
//! assert!(foldlist.remove(1,2).is_ok());
//! ```
//!
use std::collections::BTreeMap;
use std::collections::btree_map::Entry;
use std::io::{Error, ErrorKind};

use neovim_lib::{Neovim, NeovimApi};

/// An enum to denote the several types of cards a line might belong to. For now
/// carries only information equivalent to the keyword, not the subtypes, e.g.
/// CNTAC types 33 and 36 will both be denoted by type Cntac
#[derive(Clone)]
pub enum CardType {
  Node,
  Shell,
}

/// Holds the fold data of the buffer. A fold has the following data:
/// Linenumbers start, end (indexed from 1), and a
/// [CardType](enum.CardType.html).
pub struct FoldList {
  /// List of folds, keyed by [start, end], valued by CardType, sorted
  /// lexicographically on [start, end].
  folds: BTreeMap<[u64; 2], CardType>,
  /// List of folds, keyed by [end, start], valued by CardType, sorted
  /// lexicographically on [end, start].  Kept synchronous to Folds by the
  /// struct methods.
  folds_inv: BTreeMap<[u64; 2], CardType>,
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
    card: CardType,
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
  pub fn recreate_all(
    &mut self,
    lines: &Vec<String>,
  ) -> Result<(), Box<Error>> {
    debug!(
      "Veclen: {}, last line: {{ {} }}",
      lines.len(),
      lines[lines.len() - 1]
    );
    self.clear();
    let mut firstline: u64 = 0;
    let mut card: Option<CardType> = None;

    for (i, ref line) in lines.iter().enumerate() {
      if card.is_none() {
        if line.starts_with("NODE") {
          card = Some(CardType::Node);
        } else if line.starts_with("SHELL") {
          card = Some(CardType::Shell);
        }
        firstline = i as u64;
      } else {
        match card.clone().unwrap() {
          c @ CardType::Node => {
            if !line.starts_with("N") {
              self.insert(firstline + 1, i as u64, c)?;
              firstline = 0;
              card = None;
            }
          }
          c @ CardType::Shell => {
            if !line.starts_with("S") {
              self.insert(firstline + 1, i as u64, c)?;
              firstline = 0;
              card = None;
            }
          }
        }
      }
    }
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
}
