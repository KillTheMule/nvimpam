//! This module provides the [`FoldList`](::folds::FoldList) struct to
//! manage folds in a buffer. It carries both level 1 folds as well as level 2
//! folds (i.e. folds that contain folds of level 1). All functions that
//! insert/remove/modify folds operate on level 1 folds, the only thing to be
//! done for the level 2 folds is regenerating them in full from the level 1
//! folds.
//!
//! Example usage:
//!
//! A datastructure to hold the parsed data belonging to a buffer.

pub mod folds;
pub mod highlights;

use std::mem;

use failure::{Error, ResultExt};

use neovim_lib::{Neovim, NeovimApi, Value};

use crate::{
  bufdata::{folds::Folds, highlights::Highlights},
  card::keyword::Keyword,
  lines::{Line, ParsedLine},
  nocommentiter::CommentLess,
};

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

#[derive(Default, Debug)]
pub struct BufData {
  pub folds: Folds,
  pub folds_level2: Folds,
  pub highlights: Highlights,
}

impl BufData {
  pub fn new() -> BufData {
    BufData {
      folds: Folds::new(),
      folds_level2: Folds::new(),
      highlights: Highlights::new(),
    }
  }

  pub fn clear(&mut self) {
    self.folds.clear();
    self.folds_level2.clear();
    self.highlights.clear();
  }

  // TODO: Pass newfolds by value
  pub fn splice(
    &mut self,
    mut newfolds: BufData,
    firstline: usize,
    lastline: usize,
    added: i64,
  ) {
    let hl = mem::replace(&mut newfolds.highlights, Default::default());
    self.highlights.splice(
      hl,
      firstline,
      lastline,
      added,
    );

    self
      .folds
      .splice(newfolds.folds, firstline, lastline, added);

    let _ = self.folds_level2.recreate_level2(&self.folds);
  }

  /// Remove all the entries from the FoldList, and iterate over lines to
  /// populate it with new ones. Then recreate the [level 2
  /// folds](::bufdata::BufData::folds_level2).
  pub fn recreate_all(
    &mut self,
    keywords: &[Option<Keyword>],
    lines: &[Line],
  ) -> Result<(), Error> {
    self.clear();
    self.add_from(keywords, lines)?;
    self.folds_level2.recreate_level2(&self.folds)
  }

  /// Parse an array of `Option<Keyword>`s into a
  /// [`BufData`](::bufdata::BufData) struct. The computed folds and highlights
  /// are inserted in ascending order.
  ///
  /// Creates only level 1 folds. Depending on the
  /// [`ownfold`](::card::Card::ownfold) parameter in the
  /// definition of the card in the [carddata](::carddata) module, each card
  /// will be in an own fold, or several adjacent (modulo comments) cards will
  /// be subsumed into a fold.
  pub fn add_from(
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

      self
        .folds
        .checked_insert(foldstart as u64, foldend as u64, *foldkw)?;

      if let Some(Some(kl)) =
        skipped.nextline.map(|pl| pl.try_into_keywordline())
      {
        nextline = kl;
      } else {
        nextline = unwrap_or_ok!(li.skip_to_next_keyword());
      }
    }
  }

  /// Delete all folds in nvim, and create the ones from the FoldList.
  pub fn resend_all_folds(&self, nvim: &mut Neovim) -> Result<(), Error> {
    let luafn = "require('nvimpam').update_folds(...)";
    let mut luaargs = vec![];

    for (range, (_, text)) in self.folds.iter().chain(self.folds_level2.iter())
    {
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
}
