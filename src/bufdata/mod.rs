//! This module provides the [`BufData`](::bufdata::BufData) struct to
//! manage the lines, folds and highlights in a buffer.

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

/// The datastructure to hold all the information of a buffer.
#[derive(Default, Debug)]
pub struct BufData<'a> {
  /// The lines of the buffer
  pub lines: Lines<'a>,
  /// The keywords of the buffer as parsed from the lines.
  pub keywords: Keywords,
  /// The level 1 folds.
  pub folds: Folds,
  /// The level 2 folds.
  pub folds_level2: Folds,
  /// The highlights of the buffer
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

  /// Extend the lines of the buffer by splitting the slice on newlines. Parse
  /// for new keywords, and update the folds/highlights appropriately.
  ///
  /// Assumes the `BufData` was empty before.
  pub fn parse_slice<'c: 'a>(&mut self, v: &'c [u8]) -> Result<(), Error> {
    self.lines.parse_slice(v);
    self.keywords.parse_lines(&self.lines);
    self.regenerate()?;

    Ok(())
  }

  /// Extend the lines of the buffer by the `String`s in the `Vec`. Parse
  /// for new keywords, and update the folds/highlights appropriately.
  ///
  /// Assumes the `BufData` was empty before.
  pub fn parse_vec(&mut self, v: Vec<String>) -> Result<(), Error> {
    self.lines.parse_vec(v);
    self.keywords.parse_lines(&self.lines);
    self.regenerate()?;

    Ok(())
  }

  /// Extend the lines of the buffer by the `&str`s in the `slice`. Parse
  /// for new keywords, and update the folds/highlights appropriately.
  ///
  /// Assumes the `BufData` was empty before.
  pub fn parse_strs<'c: 'a>(&mut self, v: &'c [&'a str]) -> Result<(), Error> {
    self.lines.parse_strs(v);
    self.keywords.parse_lines(&self.lines);
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
  /// be as efficient as possible. Returns a tuple `Some((s, e))` such that
  /// `s..e` is the range of the indices of the highlight entries which are new.
  /// This is usefull to call
  /// [`indexrange`](::bufdata::highlights::Highlights::indexrange) afterwards
  /// to efficiently send the new data to neovim via
  /// [`highlight_region`](::bufdata::highlights::highlight_region).
  pub fn update(
    &mut self,
    firstline: i64,
    lastline: i64,
    linedata: Vec<String>,
  ) -> Result<(usize, usize), Error> {
    let added: i64 = linedata.len() as i64 - (lastline - firstline);
    self.keywords.update(firstline, lastline, &linedata);
    self.lines.update(firstline, lastline, linedata);

    let first = self.keywords.first_before(firstline);
    let last = self.keywords.first_after(lastline + added);
    let mut newhls = Highlights::default();
    let mut newfolds = Folds::default();

    // this is enumerate with i64 instead of usize
    let li = (0_i64..)
      .zip(
        self.keywords[first as usize..last as usize]
          .iter()
          .zip(self.lines[first as usize..last as usize].iter()),
      )
      .map(ParsedLine::from)
      .remove_comments();

    BufData::parse_from_iter(&mut newhls, &mut newfolds, li)?;
    self.folds.splice(newfolds, first, last, added);
    let _ = self.folds_level2.recreate_level2(&self.folds);
    Ok(self.highlights.splice(newhls, first, last, added))
  }

  /// After initializing the lines and keywords of a `BufData` structure, this
  /// finally parses them into highlights/folds. Only useful for the initial
  /// parse.
  ///
  /// TODO(KillTheMule): Can we merge this with update?
  pub fn parse_lines(&mut self) -> Result<(), Error> {
    debug_assert!(self.keywords.len() == self.lines.len());
    let li = (0_i64..)
      .zip(self.keywords.iter().zip(self.lines.iter()))
      .map(ParsedLine::from)
      .remove_comments();

    BufData::parse_from_iter(&mut self.highlights, &mut self.folds, li)
  }

  /// Iterate over a [`NoCommentIter`](::nocommentiter::NoCommentIter) and add
  /// the highlights and folds to the given structures.
  pub fn parse_from_iter<'b, I>(
    highlights: &mut Highlights,
    folds: &mut Folds,
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
      foldend = skipped.skip_end;

      folds.checked_insert(foldstart, foldend, *foldkw)?;

      if let Some(Some(kl)) =
        skipped.nextline.map(|pl| pl.try_into_keywordline())
      {
        nextline = kl;
      } else {
        nextline = unwrap_or_ok!(li.skip_to_next_keyword());
      }
    }
  }

  /// Pack up all existing level 1 and level 2 folds (in that order) into a
  /// `Value` suitable to send to neovim.
  pub fn packup_all_folds(&self) -> Value {
    let mut luaargs = vec![];

    for (range, (_, text)) in self.folds.iter().chain(self.folds_level2.iter())
    {
      luaargs.push(Value::from(vec![
        Value::from(range[0] + 1),
        Value::from(range[1] + 1),
        Value::from(text.to_string()),
      ]));
    }

    Value::from(luaargs)
  }

  /// Delete all folds in nvim, and create the ones from the `BufData`.
  pub fn resend_all_folds(&self, nvim: &mut Neovim) -> Result<(), Error> {
    let luafn = "require('nvimpam').update_folds(...)";
    let foldvalue = self.packup_all_folds();

    nvim
      .execute_lua(luafn, vec![foldvalue])
      .context("Execute lua failed")?;

    Ok(())
  }
}
