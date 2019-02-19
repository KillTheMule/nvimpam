//! This module provides the [`BufData`](::bufdata::BufData) struct to
//! manage the lines, folds and highlights in a buffer.

pub mod folds;
pub mod highlights;

use std::ops::Range;

use failure::Error;

use neovim_lib::{neovim_api::Buffer, Value};

use crate::{
  bufdata::{folds::Folds, highlights::Highlights},
  linenr::LineNr,
  lines::{Lines, ParsedLine},
  linesiter::LinesIter,
};

#[cfg(test)]
use crate::card::keyword::Keyword;

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

/// The datastructure to hold all the information of a buffer.
// TODO(KillTheMule): This needs to hold the current buffer, then make
// highlights_region etc methods on BufData
pub struct BufData<'a> {
  /// The buffer the plugin is attached to
  buf: &'a Buffer,
  /// The lines of the buffer
  lines: Lines<'a>,
  /// The level 1 folds.
  folds: Folds,
  /// The level 2 folds.
  folds_level2: Folds,
  /// The highlights of the buffer
  pub highlights: Highlights,
}

impl<'a> BufData<'a> {
  /// Create a new BufData instance for the given Neovim instance. Calls
  /// `nvim_get_current_buf` to get the buffer we're attaching to.
  ///
  /// TODO(KillTheMule): Can we take `Neovim` by value here?
  pub fn new(buf: &'a Buffer) -> Self {
    BufData {
      buf,
      lines: Lines::new(),
      folds: Folds::new(),
      folds_level2: Folds::new(),
      highlights: Highlights::new(),
    }
  }

  pub fn clear(&mut self) {
    self.lines.clear();
    self.folds.clear();
    self.folds_level2.clear();
    self.highlights.clear();
  }

  /// Extend the lines of the buffer by splitting the slice on newlines. Parse
  /// for new keywords, and update the folds/highlights appropriately.
  ///
  /// Assumes the `BufData` was empty before.
  pub fn parse_slice<'c: 'a>(&mut self, v: &'c [u8]) -> Result<(), Error> {
    self.lines.parse_slice(v);
    self.regenerate()?;

    Ok(())
  }

  /// Extend the lines of the buffer by the `String`s in the `Vec`. Parse
  /// for new keywords, and update the folds/highlights appropriately.
  ///
  /// Assumes the `BufData` was empty before.
  pub fn parse_vec(&mut self, v: Vec<String>) -> Result<(), Error> {
    self.lines.parse_vec(v);
    self.regenerate()?;

    Ok(())
  }

  /// Extend the lines of the buffer by the `&str`s in the `slice`. Parse
  /// for new keywords, and update the folds/highlights appropriately.
  ///
  /// Assumes the `BufData` was empty before.
  pub fn parse_strs<'c: 'a>(&mut self, v: &'c [&'a str]) -> Result<(), Error> {
    self.lines.parse_strs(v);
    self.regenerate()?;

    Ok(())
  }

  /// After adding lines and the keywords of a `BufData` structure, this
  /// computes the folds and highlights. Everything's cleared beforehand, so it
  /// should only be used after the initalization. Use
  /// [`update`](::bufdata::BufData::update) otherwise.
  pub fn regenerate(&mut self) -> Result<(), Error> {
    self.folds.clear();
    self.folds_level2.clear();
    self.highlights.clear();

    self.parse_lines()?;
    self.folds_level2.recreate_level2(&self.folds)?;

    Ok(())
  }

  /// Update the `BufData` structure from the lines of a `Vec<String>`. Tries to
  /// be as efficient as possible. Returns the range of indices with new
  /// highlights. This is usefull to call
  /// [`indexrange`](::bufdata::highlights::Highlights::indexrange) afterwards
  /// to efficiently send the new data to neovim via
  /// [`highlight_region`](::bufdata::highlights::highlight_region).
  pub fn update(
    &mut self,
    firstline: LineNr,
    lastline: LineNr,
    linedata: Vec<String>,
  ) -> Result<Range<usize>, Error> {
    let added: isize = linedata.len() as isize - (lastline - firstline);
    self.lines.update(firstline, lastline, linedata);

    let first = self.lines.first_before(firstline);
    let mut last = self.lines.first_after(lastline + added);
    // If lastline was the last line of the file, we need to up the index by 1
    // to include the line
    if self.lines.len() == (lastline + added).into() {
      last = (last.0 + 1, lastline + added);
    }

    let mut newhls = Highlights::new();
    let mut newfolds = Folds::new();

    let li = LinesIter::new(self.lines[first.0..last.0].iter());

    BufData::parse_from_iter(&mut newhls, &mut newfolds, li)?;
    self.folds.splice(newfolds, first.1, last.1, added);
    self.folds_level2.recreate_level2(&self.folds)?;
    Ok(self.highlights.splice(newhls, firstline, lastline, added))
  }

  /// After initializing the lines and keywords of a `BufData` structure, this
  /// finally parses them into highlights/folds. Only useful for the initial
  /// parse.
  ///
  /// TODO(KillTheMule): Can we merge this with update?
  pub fn parse_lines(&mut self) -> Result<(), Error> {
    let li = self.lines.iter();

    BufData::parse_from_iter(&mut self.highlights, &mut self.folds, li)
  }

  /// Iterate over a [`NoCommentIter`](::nocommentiter::NoCommentIter) and add
  /// the highlights and folds to the given structures.
  fn parse_from_iter<'b, I>(
    highlights: &mut Highlights,
    folds: &mut Folds,
    mut li: LinesIter<'b, I>,
  ) -> Result<(), Error>
  where
    I: Iterator<Item = &'b ParsedLine<'b>>,
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
      foldend = skipped.skip_end;

      folds.checked_insert(foldstart, foldend, foldkw)?;

      if let Some(Some(kl)) =
        skipped.nextline.map(|pl| pl.try_into_keywordline())
      {
        nextline = kl;
      } else {
        nextline = unwrap_or_ok!(li.skip_to_next_keyword());
      }
    }
  }

  pub fn hl_linerange(&self, first: LineNr, last: LineNr) -> Range<usize> {
    self.highlights.linerange(first, last)
  }

  pub fn first_before(&self, line: LineNr) -> (usize, LineNr) {
    self.lines.first_before(line)
  }

  pub fn first_after(&self, line: LineNr) -> (usize, LineNr) {
    self.lines.first_after(line)
  }

  pub fn highlight_region_calls(
    &mut self,
    indexrange: Range<usize>,
    firstline: LineNr,
    lastline: LineNr,
  ) -> Option<Vec<Value>> {
    self
      .highlights
      .highlight_region_calls(&self.buf, indexrange, firstline, lastline)
  }

  /// Pack up all existing level 1 and level 2 folds (in that order) into a
  /// `Value` suitable to send to neovim.
  pub fn fold_calls(&self) -> Value {
    Value::from(vec![
      self.folds.fold_calls(),
      self.folds_level2.fold_calls(),
    ])
  }

  #[cfg(test)]
  pub fn folds_to_vec(&self) -> Vec<(usize, usize, Keyword)> {
    self.folds.to_vec()
  }

  #[cfg(test)]
  pub fn folds_level2_to_vec(&self) -> Vec<(usize, usize, Keyword)> {
    self.folds_level2.to_vec()
  }
}
