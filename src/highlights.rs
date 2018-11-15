//! The highlight module
use std::{self, cmp, convert::From};

use card::{cell::Cell, line::Line as CardLine};
use highlights::HighlightGroup as Hl;

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
    } else if self
      .line
      .text
      .get(range.clone())
      .map(|s| cell.verify(s))
      .is_some()
    {
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
    } else {
      None
    }
  }
}
