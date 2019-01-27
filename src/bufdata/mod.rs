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

use failure::{Error, ResultExt};

use neovim_lib::{Neovim, NeovimApi, Value};

use crate::{
  bufdata::{folds::Folds, highlights::Highlights},
  card::keyword::Keywords,
  lines::{Lines, ParsedLine},
  nocommentiter::{CommentLess, NoCommentIter},
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
pub struct BufData<'a> {
  pub lines: Lines<'a>,
  pub keywords: Keywords,
  pub folds: Folds,
  pub folds_level2: Folds,
  pub highlights: Highlights,
}

impl<'a> BufData<'a> {
  pub fn new() -> Self {
    BufData {
      lines: Lines::new(),
      keywords: Keywords::new(),
      folds: Folds::new(),
      folds_level2: Folds::new(),
      highlights: Highlights::new(),
    }
  }

  pub fn clear(&mut self) {
    self.lines.clear();
    self.keywords.clear();
    self.folds.clear();
    self.folds_level2.clear();
    self.highlights.clear();
  }

  // will simply push stuff, makes only sense for new empty bufdata
  pub fn from_slice<'c: 'a>(&mut self, v: &'c [u8]) {
    self.lines.from_slice(v);
    self.keywords.from_lines(&self.lines);
    self.regenerate();
  }

  // will simply push stuff, makes only sense for new empty bufdata
  pub fn from_vec(&mut self, v: Vec<String>) {
    self.lines.from_vec(v);
    self.keywords.from_lines(&self.lines);
    self.regenerate();
  }

  // will simply push stuff, makes only sense for new empty bufdata
  pub fn from_strs<'c: 'a>(&mut self, v: &'c [&'a str]) {
    self.lines.from_strs(v);
    self.keywords.from_lines(&self.lines);
    self.regenerate();
  }

  pub fn regenerate(&mut self) {
    self.folds.clear();
    self.folds_level2.clear();
    self.highlights.clear();

    self.parse_lines();
    self.folds_level2.recreate_level2(&self.folds);
  }

  pub fn update(
    &mut self,
    firstline: u64,
    lastline: u64,
    linedata: Vec<String>,
  ) -> (usize, usize) {
    let added: i64 = linedata.len() as i64 - (lastline - firstline) as i64;
    self.keywords.update(firstline, lastline, &linedata);
    self.lines.update(firstline, lastline, linedata);

    let first = self.keywords.first_before(firstline);
    let last = self.keywords.first_after((lastline as i64 + added) as u64);
    let mut newhls: Highlights = Default::default();
    let mut newfolds: Folds = Default::default();

    let li = self.keywords[first..last]
      .iter()
      .zip(self.lines[first..last].iter())
      .enumerate()
      .map(ParsedLine::from)
      .remove_comments();

    BufData::parse_from_iter(&mut newhls, &mut newfolds, self.lines.len(), li);

    self.folds.splice(newfolds, last, last, added);

    let _ = self.folds_level2.recreate_level2(&self.folds);
    self.highlights.splice(newhls, first, last, added)
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
  pub fn parse_lines(&mut self) -> Result<(), Error> {
    debug_assert!(self.keywords.len() == self.lines.len());
    let li = self
      .keywords
      .iter()
      .zip(self.lines.iter())
      .enumerate()
      .map(ParsedLine::from)
      .remove_comments();

    BufData::parse_from_iter(
      &mut self.highlights,
      &mut self.folds,
      self.lines.len(),
      li,
    )
  }

  pub fn parse_from_iter<'b, I>(
    highlights: &mut Highlights,
    folds: &mut Folds,
    len: usize,
    mut li: NoCommentIter<I>,
  ) -> Result<(), Error>
  where
    I: Iterator<Item = ParsedLine<'b>>,
  {
    let mut foldstart;
    let mut foldend;
    let mut foldkw;
    let mut skipped;

    let mut nextline = unwrap_or_ok!(li.skip_to_next_keyword());

    loop {
      foldkw = nextline.keyword;
      foldstart = nextline.number;
      skipped = li.skip_fold(&nextline, highlights);

      // The latter only happens when a file ends after the only line of a card
      foldend = skipped.skip_end.unwrap_or_else(|| len - 1);

      folds.checked_insert(foldstart as u64, foldend as u64, *foldkw)?;

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
