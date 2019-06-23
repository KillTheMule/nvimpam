//! The highlight module
use std::{self, cmp, convert::From, ops::Range};

use neovim_lib::{neovim_api::Buffer, Value};

use crate::{
  bufdata::highlights::HighlightGroup as Hl,
  card::{cell::Cell, line::Line as CardLine},
  linenr::LineNr,
};

/// An enum to denote the nvim highlight groups within nvimpam
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

/// The Iterator for a [`HlLine`](::bufdata::highlights::HlLine).
#[derive(Debug)]
struct HlIter<'a> {
  num: LineNr,
  cardline: &'a CardLine,
  text: &'a [u8],
  linelen: u8,
  until: u8,
  odd: bool,
  cells: std::slice::Iter<'a, Cell>,
}

impl<'a> Iterator for HlIter<'a> {
  type Item = ((LineNr, u8, u8), Hl);

  fn next(&mut self) -> Option<Self::Item> {
    if self.until >= self.linelen {
      return None;
    }

    let cell = match self.cells.next() {
      Some(c) => c,
      None => return None,
    };

    let mut celllen = cell.len();

    // Overwrite celllen for cells of length 0, since they're the last ones in a
    // line and we just highlight everything until the end
    if celllen == 0 {
      celllen = self.linelen - self.until;
    }

    let range = self.until..cmp::min(self.linelen, self.until + celllen);
    let odd = self.odd;

    self.until += celllen;
    self.odd = !odd;

    if let Cell::Kw(_) = cell {
      Some(((self.num, range.start, range.end), Hl::Keyword))
    } else {
      match self
        .text
        .get(range.start as usize..range.end as usize)
        .map(|s| cell.verify(s))
      {
        Some(true) => {
          if odd {
            Some(((self.num, range.start, range.end), Hl::CellEven))
          } else {
            Some(((self.num, range.start, range.end), Hl::CellOdd))
          }
        }
        Some(false) => {
          if odd {
            Some(((self.num, range.start, range.end), Hl::ErrorCellEven))
          } else {
            Some(((self.num, range.start, range.end), Hl::ErrorCellOdd))
          }
        }
        None => None,
      }
    }
  }
}

/// The struct to hold the highlights for a buffer. The internal `Vec` needs to
/// stay ordered on the first tuple.
///
/// TODO(KillTheMule): Don't expose the internal `Vec`
#[derive(Default, Debug)]
pub struct Highlights(pub Vec<((LineNr, u8, u8), Hl)>);

impl Highlights {
  pub(super) fn clear(&mut self) {
    self.0.clear()
  }

  pub fn new() -> Self {
    Self(Vec::new())
  }

  pub fn iter(&self) -> impl Iterator<Item = &((LineNr, u8, u8), Hl)> {
    self.0.iter()
  }

  /// Remove all the highlights with linenumbers in `firstline..lastline`, and
  /// paste in the ones given in `newhls`. Keeps the `Vec` ordered. Returns the
  /// range of indices with new highlight entries (note that all the elements
  /// above that range have been modified, as their line numbers had to be
  /// shifted).
  pub fn splice(
    &mut self,
    newhls: Self,
    firstline: LineNr,
    lastline: LineNr,
    added: isize,
  ) -> Range<usize> {
    let start = self
      .0
      .binary_search_by_key(&(firstline, 0), |&((l, s, _), _)| (l, s))
      // error contains index where ele could be inserted preserving Order
      .unwrap_or_else(|e| e);
    let end = self.0[start..]
      .iter()
      .enumerate()
      .find(|(_, ((l, _, _), _))| *l >= lastline)
      .map_or_else(|| self.0.len(), |(i, ((_, _, _), _))| i + start);

    let num_new = newhls.0.len();
    let _ = self.0.splice(
      start..end,
      newhls.0.into_iter().map(|((l, s, e), h)| ((l, s, e), h)),
    );

    if added != 0 {
      for t in self.0[start + num_new..].iter_mut() {
        ((*t).0).0 += added;
      }
    }

    start..(start + num_new)
  }

  /// Add the highlights of a line by pushing them to the end of the `Vec`. Be
  /// sure that the order of the `Vec` is not destroyed by this!
  #[inline]
  pub fn add_line_highlights(
    &mut self,
    num: LineNr,
    text: &[u8],
    cardline: &CardLine,
  ) {
    // We only highlight until column 81
    #![allow(clippy::cast_possible_truncation)]
    let linelen = cmp::min(text.len(), 81) as u8;
    let cells = cardline.cells().unwrap_or(&[]).iter();
    let it = HlIter {
      num,
      cardline,
      text,
      linelen,
      until: 0,
      odd: false,
      cells,
    };
    self.0.extend(it);
  }

  /// Return an iterator over the highlights of the lines with linenumber in the
  /// range `firstline..lastline`.
  pub(super) fn linerange(
    &self,
    firstline: LineNr,
    lastline: LineNr,
  ) -> Range<usize> {
    let start = self
      .0
      .binary_search_by_key(&(firstline, 0), |&((l, s, _), _)| (l, s))
      // error contains index where ele could be inserted preserving Order
      .unwrap_or_else(|e| e);
    let end = self.0[start..]
      .iter()
      .enumerate()
      .find(|(_, ((l, _, _), _))| *l >= lastline)
      .map_or_else(|| self.0.len(), |(i, ((_, _, _), _))| i + start);

    start..end
  }

  /// Construct the necessary calls to neovim to highlight a region. Here,
  /// `indexrange` gives the index of the highlights to send. All existing
  /// highlights in this linerange are cleare beforehand.
  ///
  /// TODO(KillTheMule): efficient? maybe not send strings for the hl groups,
  /// but small ints, and have a mapping ready?
  pub(super) fn highlight_region_calls(
    &self,
    buf: &Buffer,
    indexrange: Range<usize>,
  ) -> Option<Vec<Value>> {
    let hls = &self.0[indexrange.clone()];
    let firstline;
    let lastline;

    if let Some(f) = hls.first().map(|((l, _, _), _)| l) {
      firstline = f;
    } else {
      return None;
    }

    if let Some(l) = hls.last().map(|((l, _, _), _)| l) {
      lastline = l;
    } else {
      return None;
    }

    if indexrange.start == indexrange.end && firstline == lastline {
      return None;
    }

    let mut calls: Vec<Value> = vec![];

    calls.push(
      vec![
        Value::from("nvim_buf_clear_highlight".to_string()),
        vec![
          buf.get_value().clone(),
          Value::from(5),
          Value::from(*firstline),
          Value::from(*lastline),
        ]
        .into(),
      ]
      .into(),
    );

    calls.extend(hls.iter().map(|((l, s, e), t)| {
      let st: &'static str = (*t).into();
      vec![
        Value::from("nvim_buf_add_highlight".to_string()),
        vec![
          buf.get_value().clone(),
          Value::from(5),
          Value::from(st.to_string()),
          Value::from(*l),
          Value::from(u64::from(*s)),
          Value::from(u64::from(*e)),
        ]
        .into(),
      ]
      .into()
    }));

    Some(calls)
  }

  #[cfg(test)]
  pub fn add_highlight(&mut self, line: LineNr, start: u8, end: u8, hl: Hl) {
    self.0.push(((line, start, end), hl))
  }
}

#[cfg(test)]
macro_rules! splicetest {
  (
    $fn: ident;
    existing: $([$l: expr, $($e: expr),+]),+;
    new: $([$ll: expr, $($f: expr),+]),+;
    $first: expr, $last: expr, $added: expr;
    expected: $([$lll: expr, $($g: expr),+]),+
  ) => {
    #[test]
    fn $fn() {
      use crate::linenr::LineNr;

      let mut h = Highlights::new();
      $(let _ = h.add_highlight(LineNr::from_usize($l), $($e),+);)+

      let mut h1 = Highlights::new();
      $(let _ = h1.add_highlight(LineNr::from_usize($ll), $($f),+);)+

      h.splice(h1, LineNr::from_usize($first), LineNr::from_usize($last), $added);
      let v = vec![$( (LineNr::from_usize($lll), $($g),+ ),)+];

      let w:Vec<_> = h.0.iter().map(|((l, s, e), h)| (*l, *s, *e, *h)).collect();
      assert_eq!(v, w);
    }
  };
}

#[cfg(test)]
mod tests {
  use crate::bufdata::highlights::{HighlightGroup::*, Highlights};

  // adding 3 lines before the buffer
  splicetest!(hl_splice_before;
              existing:
                [0, 0, 8, Keyword],
                [0, 9, 16, CellOdd],
                [1, 0, 4, Keyword],
                [1, 5, 12, CellOdd],
                [1, 13, 20, CellEven],
                [2, 0, 8, Keyword],
                [2, 9, 16, CellOdd];
              new:
                [0, 0, 4, Keyword],
                [0, 5, 80, CellOdd];
              0, 1, 3;
              expected:
                 [0, 0, 4, Keyword],
                 [0, 5, 80, CellOdd],
                 [4, 0, 4, Keyword],
                 [4, 5, 12, CellOdd],
                 [4, 13, 20, CellEven],
                 [5, 0, 8, Keyword],
                 [5, 9, 16, CellOdd]
                 );

  // 4 lines have been pasted after the last line of the buffer
  splicetest!(hl_splice_after;
              existing:
                 [0, 0, 4, Keyword],
                 [0, 5, 80, CellOdd],
                 [4, 0, 4, Keyword],
                 [4, 5, 12, CellOdd],
                 [4, 13, 20, CellEven],
                 [5, 0, 8, Keyword],
                 [5, 9, 16, CellOdd];
              new:
                 [6, 0, 8, Keyword],
                 [9, 0, 8, CellOdd],
                 [9, 9, 16, CellEven],
                 [9, 17, 24, CellOdd];
              6, 6, 4;
              expected:
                 [0, 0, 4, Keyword],
                 [0, 5, 80, CellOdd],
                 [4, 0, 4, Keyword],
                 [4, 5, 12, CellOdd],
                 [4, 13, 20, CellEven],
                 [5, 0, 8, Keyword],
                 [5, 9, 16, CellOdd],
                 [6, 0, 8, Keyword],
                 [9, 0, 8, CellOdd],
                 [9, 9, 16, CellEven],
                 [9, 17, 24, CellOdd]
          );

  // changing one line
  splicetest!(hl_splice_change_one_line;
              existing:
                 [0, 0, 4, Keyword],
                 [0, 5, 80, CellOdd],
                 [1, 0, 4, Keyword],
                 [1, 5, 12, CellOdd],
                 [1, 13, 20, CellEven],
                 [2, 0, 8, Keyword],
                 [2, 9, 16, CellOdd],
                 [3, 0, 8, Keyword],
                 [3, 9, 16, CellOdd];
              new:
                 [1, 0, 8, Keyword];
              1, 2, 0;
              expected:
                 [0, 0, 4, Keyword],
                 [0, 5, 80, CellOdd],
                 [1, 0, 8, Keyword],
                 [2, 0, 8, Keyword],
                 [2, 9, 16, CellOdd],
                 [3, 0, 8, Keyword],
                 [3, 9, 16, CellOdd]
          );

  // delete 1 line, insert 2
  splicetest!(hl_splice_add_one_line;
              existing:
                 [0, 0, 4, Keyword],
                 [0, 5, 80, CellOdd],
                 [1, 0, 4, Keyword],
                 [1, 5, 12, CellOdd],
                 [1, 13, 20, CellEven],
                 [2, 0, 8, Keyword],
                 [2, 9, 16, CellOdd],
                 [3, 0, 8, Keyword],
                 [3, 9, 16, CellOdd];
              new:
                 [1, 0, 8, Keyword],
                 [1, 9, 16, CellOdd],
                 [2, 0, 8, Keyword],
                 [2, 9, 12, CellOdd];
              1, 2, 1;
              expected:
                 [0, 0, 4, Keyword],
                 [0, 5, 80, CellOdd],
                 [1, 0, 8, Keyword],
                 [1, 9, 16, CellOdd],
                 [2, 0, 8, Keyword],
                 [2, 9, 12, CellOdd],
                 [3, 0, 8, Keyword],
                 [3, 9, 16, CellOdd],
                 [4, 0, 8, Keyword],
                 [4, 9, 16, CellOdd]
          );

  // delete 2 lines, insert 1
  splicetest!(hl_splice_delete_one_line;
              existing:
                 [0, 0, 4, Keyword],
                 [0, 5, 80, CellOdd],
                 [1, 0, 4, Keyword],
                 [1, 5, 12, CellOdd],
                 [1, 13, 20, CellEven],
                 [2, 0, 8, Keyword],
                 [2, 9, 16, CellOdd],
                 [3, 0, 8, Keyword],
                 [3, 9, 16, CellOdd],
                 [4, 0, 8, Keyword];
              new:
                 [2, 0, 8, Keyword],
                 [2, 9, 16, CellOdd];
              2, 4, -1;
              expected:
                 [0, 0, 4, Keyword],
                 [0, 5, 80, CellOdd],
                 [1, 0, 4, Keyword],
                 [1, 5, 12, CellOdd],
                 [1, 13, 20, CellEven],
                 [2, 0, 8, Keyword],
                 [2, 9, 16, CellOdd],
                 [3, 0, 8, Keyword]
          );

  // overwrite the last 2 lines with 4 lines
  splicetest!(hl_splice_overwrite_end;
              existing:
                 [0, 0, 4, Keyword],
                 [0, 5, 80, CellOdd],
                 [1, 0, 4, Keyword],
                 [1, 5, 12, CellOdd],
                 [1, 13, 20, CellEven],
                 [2, 0, 8, Keyword],
                 [2, 9, 16, CellOdd];
              new:
                 [1, 0, 8, Keyword],
                 [1, 9, 16, CellOdd],
                 [2, 0, 8, Keyword],
                 [3, 0, 8, Keyword],
                 [4, 0, 8, Keyword];
              1, 3, 2;
              expected:
                 [0, 0, 4, Keyword],
                 [0, 5, 80, CellOdd],
                 [1, 0, 8, Keyword],
                 [1, 9, 16, CellOdd],
                 [2, 0, 8, Keyword],
                 [3, 0, 8, Keyword],
                 [4, 0, 8, Keyword]
          );

  // overwrite the first 2 lines with 4 lines
  splicetest!(hl_splice_overwrite_start;
              existing:
                 [0, 0, 4, Keyword],
                 [0, 5, 80, CellOdd],
                 [1, 0, 4, Keyword],
                 [1, 5, 12, CellOdd],
                 [1, 13, 20, CellEven],
                 [2, 0, 8, Keyword],
                 [2, 9, 16, CellOdd];
              new:
                 [0, 0, 8, Keyword],
                 [0, 9, 16, CellOdd],
                 [1, 0, 8, Keyword],
                 [2, 0, 8, Keyword],
                 [3, 0, 8, Keyword];
              0, 2, 2;
              expected:
                 [0, 0, 8, Keyword],
                 [0, 9, 16, CellOdd],
                 [1, 0, 8, Keyword],
                 [2, 0, 8, Keyword],
                 [3, 0, 8, Keyword],
                 [4, 0, 8, Keyword],
                 [4, 9, 16, CellOdd]
          );

  #[test]
  pub fn hl_iteration_order() {
    let mut h = Highlights::new();

    h.add_highlight(0.into(), 0, 8, Keyword);
    h.add_highlight(0.into(), 9, 16, CellOdd);
    h.add_highlight(1.into(), 0, 4, Keyword);
    h.add_highlight(1.into(), 5, 12, CellOdd);
    h.add_highlight(1.into(), 13, 20, CellEven);
    h.add_highlight(2.into(), 0, 8, Keyword);
    h.add_highlight(2.into(), 9, 16, CellOdd);

    let v = vec![
      (0.into(), 0, 8, Keyword),
      (0.into(), 9, 16, CellOdd),
      (1.into(), 0, 4, Keyword),
      (1.into(), 5, 12, CellOdd),
      (1.into(), 13, 20, CellEven),
      (2.into(), 0, 8, Keyword),
      (2.into(), 9, 16, CellOdd),
    ];

    // this is not a trivial test, it ascertains the iteration order
    let w: Vec<_> = h.0.iter().map(|((l, s, e), h)| (*l, *s, *e, *h)).collect();
    assert_eq!(v, w);
  }

}
