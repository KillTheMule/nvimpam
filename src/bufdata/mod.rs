//! This module provides the [`BufData`](crate::bufdata::BufData) struct to
//! manage the lines, folds and highlights in a buffer.

pub mod folds;
pub mod highlights;

use std::ops::Range;

use failure::Error;

use neovim_lib::{neovim_api::Buffer, Value};

use crate::{
  bufdata::{folds::Folds, highlights::Highlights},
  card::{line::Line as CardLine, Card},
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
pub struct BufData<'a> {
  /// The buffer the plugin is attached to
  pub buf: &'a Buffer,
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
  /// Create a new BufData instance for a given buffer.
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
  /// [`update`](crate::bufdata::BufData::update) otherwise.
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
  /// [`highlight_region_calls`](crate::bufdata::BufData::
  /// highlight_region_calls) afterwards.
  pub fn update(
    &mut self,
    firstline: LineNr,
    lastline: LineNr,
    linedata: Vec<String>,
  ) -> Result<(Range<usize>, isize), Error> {
    let added: isize = linedata.len() as isize - (lastline - firstline);

    // the old behavior, just keep that for now
    let mut first_pre =
      self.lines.first_before(firstline).unwrap_or_else(|| {
        self
          .lines
          .get(0)
          .map_or((0, 0_usize.into()), |l| (0, l.number))
      });

    // the old behavior, just keep that for now
    let last_pre = self.lines.first_after(lastline).unwrap_or_else(|| {
      if self.lines.is_empty() {
        (0_usize, 0_usize.into())
      } else {
        let len = self.lines.len();
        (len, self.lines[len - 1].number + 1)
      }
    });

    let adjust_first = self
      .lines
      .last()
      .map(|l| l.number < firstline)
      .unwrap_or(false);

    if adjust_first {
      // firstline is after the last line of the file, so we got back the
      // last line's data, but we want the virtual one after that
      // TODO(KillTheMule): Should this be last_pre? Why does it work then?
      first_pre.0 += 1;
      first_pre.1 += 1;
    }

    let added_nocom = self.lines.update(linedata, firstline, lastline, added);

    let first_post = first_pre.0;
    // TODO(KillTheMule): Check this!
    let last_post = ((last_pre.0 as isize) + added_nocom) as usize;

    let mut newhls = Highlights::new();
    let mut newfolds = Folds::new();

    let li = LinesIter::new(self.lines[first_post..last_post].iter());

    BufData::parse_from_iter(&mut newhls, &mut newfolds, li)?;
    self.folds.splice(newfolds, first_pre.1, last_pre.1, added);
    self.folds_level2.recreate_level2(&self.folds)?;
    Ok((
      self.highlights.splice(newhls, firstline, lastline, added),
      added,
    ))
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

  /// Iterate over a [`LinesIter`](::linesiter::LinesIter) and add
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
        skipped.nextline.map(ParsedLine::try_into_keywordline)
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

  pub fn first_before(&self, line: LineNr) -> Option<(usize, LineNr)> {
    self.lines.first_before(line)
  }

  pub fn first_after(&self, line: LineNr) -> Option<(usize, LineNr)> {
    self.lines.first_after(line)
  }

  /// Construct the necessary calls to neovim to highlight the region given by
  /// `firstline..lastline`. Here, `indexrange` gives the index of the
  /// highlights to send. All existing highlights in this linerange are cleare
  /// beforehand.
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

  pub fn cellhint(&self, line: LineNr, column: u8) -> Value {
    // TODO(KillTheMule): This must be more efficient
    let empty_array = Value::from(vec![Value::from(""), Value::from("")]);

    let clineidx = match self.first_before(line) {
      Some(c) => c,
      None => return empty_array,
    }
    .0;
    let mut it = self.lines.iter_from(clineidx);

    let cline = match it.next().and_then(ParsedLine::try_into_keywordline) {
      Some(kl) => kl,
      None => return empty_array,
    };

    /*
    .ok_or_else(|| {
      failure::err_msg(format!(
        "Index {} of BufData.lines does not contain \
         a keywordline, although it was returned by \
         self.first_before({})!",
        clineidx, line
      ))
    })?;
    */
    let card: &'static Card = (&cline.keyword).into();

    let cardline: &CardLine = match it.get_cardline_by_nr(&cline, card, line) {
      Some(c) => c,
      None => return empty_array,
    };
    /*
    let cardlineidx: u8 =
      match it.get_cardline_hints_index(&cline, card, line) {
        Some(u) => u,
        None => return Value::from("")
      };
        .ok_or_else(|| {
          failure::err_msg(format!(
            "Card {:?} on the line with index {} does not contain the line {}, \
             even though it is the last card before it!",
             card.keyword(), clineidx, line
           ))
        })?;

    let cardhint: &'static CardHint = card.into();
    let linehint: &'static LineHint = cardhint.get(cardlineidx);
    let cellhint: Option<&'static CellHint> = linehint.get_cell(column);
        */

    let hint: &str = cardline.hint(column);
    let kw: &str = (&cline.keyword).into();

    //Value::from(cellhint.map(|c| c.id()).unwrap_or(""))
    Value::from(vec![Value::from(kw), Value::from(hint)])
  }

  pub fn firstline_number(&self) -> LineNr {
    self
      .lines
      .get(0)
      .map(|l| l.number)
      .unwrap_or_else(|| LineNr::from(0))
  }

  pub fn lastline_number(&self) -> LineNr {
    self
      .lines
      .iter()
      .last()
      .map(|l| l.number)
      .unwrap_or_else(|| LineNr::from(0))
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
