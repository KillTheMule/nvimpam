//! The highlight module
use std::{
  self, cmp,
  collections::{btree_map::Entry, BTreeMap},
  convert::From,
};

use failure::{Error, ResultExt};
use neovim_lib::{Neovim, NeovimApi, Value};

use crate::{
  bufdata::highlights::HighlightGroup as Hl,
  card::{cell::Cell, line::Line as CardLine},
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum HighlightGroup {
  CellEven,
  CellOdd,
  ErrorCellEven,
  ErrorCellOdd,
  Keyword,
}

impl From<HighlightGroup> for &'static str {
  fn from(h: HighlightGroup) -> &'static str {
    use self::HighlightGroup::*;

    match h {
      CellEven => "PamCellEven",
      CellOdd => "PamCellOdd",
      ErrorCellEven => "PamErrorCellEven",
      ErrorCellOdd => "PamErrorCellOdd",
      Keyword => "PamKeyword",
    }
  }
}

#[derive(Debug)]
pub struct HlLine<'a> {
  pub cardline: &'a CardLine,
  pub num: usize,
  pub text: &'a [u8],
}

impl<'a> IntoIterator for HlLine<'a> {
  type Item = ((u64, u8, u8), Hl);
  type IntoIter = HlIter<'a>;

  fn into_iter(self) -> Self::IntoIter {
    let linelen = self.text.len();
    let cells = self.cardline.cells().unwrap_or(&[]).iter();

    HlIter {
      line: self,
      linelen,
      until: 0,
      odd: false,
      cells,
    }
  }
}
#[derive(Debug)]
pub struct HlIter<'a> {
  line: HlLine<'a>,
  linelen: usize,
  until: u8,
  odd: bool,
  cells: std::slice::Iter<'a, Cell>,
}

impl<'a> Iterator for HlIter<'a> {
  type Item = ((u64, u8, u8), Hl);

  fn next(&mut self) -> Option<Self::Item> {
    if self.until as usize >= self.linelen {
      return None;
    }

    let cell = match self.cells.next() {
      Some(c) => c,
      None => return None,
    };

    let celllen = cell.len();
    let range = self.until as usize
      ..cmp::min(self.linelen, (self.until + celllen) as usize);
    let odd = self.odd;

    self.until += celllen;
    self.odd = !odd;

    if let Cell::Kw(_) = cell {
      Some((
        (self.line.num as u64, range.start as u8, range.end as u8),
        Hl::Keyword,
      ))
    } else {
      match self.line.text.get(range.clone()).map(|s| cell.verify(s)) {
        Some(true) => {
          if odd {
            Some((
              (self.line.num as u64, range.start as u8, range.end as u8),
              Hl::CellEven,
            ))
          } else {
            Some((
              (self.line.num as u64, range.start as u8, range.end as u8),
              Hl::CellOdd,
            ))
          }
        }
        Some(false) => {
          if odd {
            Some((
              (self.line.num as u64, range.start as u8, range.end as u8),
              Hl::ErrorCellEven,
            ))
          } else {
            Some((
              (self.line.num as u64, range.start as u8, range.end as u8),
              Hl::ErrorCellOdd,
            ))
          }
        }
        None => None,
      }
    }
  }
}

#[derive(Default, Debug)]
pub struct Highlights(BTreeMap<(u64, u8, u8), Hl>);

impl Highlights {
  pub fn clear(&mut self) {
    self.0.clear()
  }

  pub fn new() -> Self {
    Highlights(BTreeMap::new())
  }

  pub fn splice(
    &mut self,
    newfolds: &mut Highlights,
    firstline: usize,
    lastline: usize,
    added: i64,
  ) {
    let first_to_delete = self
      .0
      .range((firstline as u64, 0, 0)..)
      .next()
      .map(|f| *(f.0));

    let mut to_change = match first_to_delete {
      Some(ftd) => self.0.split_off(&ftd),
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

    for (k, v) in newfolds.0.iter() {
      self.add_highlight(k.0 + firstline as u64, k.1, k.2, *v);
    }

    for (k, v) in to_move.iter() {
      self.add_highlight((k.0 as i64 + added) as u64, k.1, k.2, *v);
    }
  }

  pub fn add_highlight(&mut self, line: u64, start: u8, end: u8, typ: Hl) {
    match self.0.entry((line, start, end)) {
      Entry::Vacant(entry) => {
        entry.insert(typ);
      }
      Entry::Occupied(mut entry) => {
        *entry.get_mut() = typ;
      }
    }
  }

  pub fn extend<T>(&mut self, it: T)
  where
    T: IntoIterator<Item = ((u64, u8, u8), Hl)>,
  {
    self.0.extend(it)
  }

  pub fn iter(&self) -> impl Iterator<Item = (&(u64, u8, u8), &Hl)> {
    self.0.iter()
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

    for ((l, s, e), t) in self.0.iter() {
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
}
