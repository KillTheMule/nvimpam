//! The highlight module
use std::{self, cmp, convert::From};

use failure::{Error, ResultExt};
use neovim_lib::{Neovim, NeovimApi, Value};

use crate::{
  bufdata::highlights::HighlightGroup as Hl,
  card::{cell::Cell, line::Line as CardLine},
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

/// A struct for a line that should get highlighting applied to it. Its main use
/// is as an `Iterator` over the highlights of that line.
#[derive(Debug)]
pub struct HlLine<'a> {
  pub cardline: &'a CardLine,
  pub text: &'a [u8],
}

impl<'a> IntoIterator for HlLine<'a> {
  type Item = ((u8, u8), Hl);
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

/// The Iterator for a [`HlLine`](::bufdata::highlights::HlLine).
#[derive(Debug)]
pub struct HlIter<'a> {
  line: HlLine<'a>,
  linelen: usize,
  until: u8,
  odd: bool,
  cells: std::slice::Iter<'a, Cell>,
}

impl<'a> Iterator for HlIter<'a> {
  type Item = ((u8, u8), Hl);

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
      Some(((range.start as u8, range.end as u8), Hl::Keyword))
    } else {
      match self.line.text.get(range.clone()).map(|s| cell.verify(s)) {
        Some(true) => {
          if odd {
            Some(((range.start as u8, range.end as u8), Hl::CellEven))
          } else {
            Some(((range.start as u8, range.end as u8), Hl::CellOdd))
          }
        }
        Some(false) => {
          if odd {
            Some(((range.start as u8, range.end as u8), Hl::ErrorCellEven))
          } else {
            Some(((range.start as u8, range.end as u8), Hl::ErrorCellOdd))
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
pub struct Highlights(pub Vec<((u64, u8, u8), Hl)>);

impl Highlights {
  pub fn clear(&mut self) {
    self.0.clear()
  }

  pub fn new() -> Self {
    Highlights(Vec::new())
  }

  /// Remove all the highlights with linenumbers in `firstline..lastline`, and
  /// paste in the ones given in `newhls`. Keeps the `Vec` ordered. Returns a
  /// tuple `(s, e)` such that `s..e` is the index range of the new highlight
  /// entries (note that all the elements in the index range `e..` have been
  /// modified, as their line numbers had to be shifted).
  pub fn splice(
    &mut self,
    newhls: Highlights,
    firstline: usize,
    lastline: usize,
    added: i64,
  ) -> (usize, usize) {
    let start = self
      .0
      .binary_search_by_key(&(firstline, 0), |&((l, s, _), _)| (l as usize, s))
      // error contains index where ele could be inserted preserving Order
      .unwrap_or_else(|e| e);
    let end = self.0[start..]
      .iter()
      .enumerate()
      .find(|(_, ((l, _, _), _))| *l as usize >= lastline)
      .map(|(i, ((_, _, _), _))| i + start)
      .unwrap_or_else(|| self.0.len());

    let num_new = newhls.0.len();
    let _ = self.0.splice(
      start..end,
      newhls
        .0
        .into_iter()
        .map(|((l, s, e), h)| ((l + firstline as u64, s, e), h)),
    );

    if added != 0 {
      for t in self.0[start + num_new..].iter_mut() {
        ((*t).0).0 = (((*t).0).0 as i64 + added) as u64;
      }
    }

    (start, start + num_new)
  }

  /// Add a highlight by pushing it to the end of the `Vec`. Be sure that the
  /// order of the `Vec` is not destroyed by this!
  pub fn add_highlight(&mut self, line: u64, start: u8, end: u8, typ: Hl) {
    self.0.push(((line, start, end), typ));
  }

  /// Add the highlights of a line by pushing them to the end of the `Vec`. Be
  /// sure that the order of the `Vec` is not destroyed by this!
  #[inline]
  pub fn add_line_highlights<T>(&mut self, num: usize, it: T)
  where
    T: IntoIterator<Item = ((u8, u8), Hl)>,
  {
    self
      .0
      .extend(it.into_iter().map(|((s, e), h)| ((num as u64, s, e), h)));
  }

  pub fn iter(&self) -> impl Iterator<Item = (&(u64, u8, u8), &Hl)> {
    self.0.iter().map(|(ref a, ref b)| (a, b))
  }

  /// Return an iterator over the highlights of the lines with index (in the
  /// internal `Vec`) in the range `firstline..lastline`.
  pub fn indexrange(
    &self,
    firstline: usize,
    lastline: usize,
  ) -> impl Iterator<Item = (&(u64, u8, u8), &Hl)> {
    self.0[firstline..lastline]
      .iter()
      .map(|(ref a, ref b)| (a, b))
  }

  /// Return an iterator over the highlights of the lines with linenumber in the
  /// range `firstline..lastline`.
  pub fn linerange(
    &self,
    firstline: u64,
    lastline: u64,
  ) -> impl Iterator<Item = (&(u64, u8, u8), &Hl)> {
    let start = self
      .0
      .binary_search_by_key(&(firstline, 0), |&((l, s, _), _)| (l, s))
      // error contains index where ele could be inserted preserving Order
      .unwrap_or_else(|e| e);
    let end = self.0[start..]
      .iter()
      .enumerate()
      .find(|(_, ((l, _, _), _))| *l >= lastline)
      .map(|(i, ((_, _, _), _))| i + start)
      .unwrap_or_else(|| self.0.len());

    self.0[start..end].iter().map(|(ref a, ref b)| (a, b))
  }
}

/// Send the lighlights from the passed Iterator to neovim. All the highlights
/// in the linerange `firstline..lastline` are cleared beforehand.
///
/// TODO(KillTheMule): efficient?
/// TODO(KillTheMule): This should be a method on BufData
pub fn highlight_region<'a, 'b, 'c, T>(
  iter: T,
  nvim: &'a mut Neovim,
  firstline: u64,
  lastline: u64,
) -> Result<(), Error>
where
  T: Iterator<Item = (&'b (u64, u8, u8), &'b Hl)>,
{
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

  calls.extend(iter.map(|((l, s, e), t)| {
    let st: &'static str = (*t).into();
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
    .into()
  }));

  nvim.call_atomic(calls).context("call_atomic failed")?;
  Ok(())
}

#[cfg(test)]
macro_rules! splicetest {
  ($fn: ident; existing: $([$($e: expr),+]),+; new: $([$($f: expr),+]),+;
  $first: expr, $last: expr, $added: expr; expected: $([$($g: expr),+]),+ ) => {
    #[test]
    fn $fn() {
      let mut h = Highlights::new();
      $(let _ = h.add_highlight($($e),+);)+

      let mut h1 = Highlights::new();
      $(let _ = h1.add_highlight($($f),+);)+

      h.splice(h1, $first, $last, $added);
      let v = vec![$( ($($g),+ ),)+];

      let w:Vec<_> = h.iter().map(|((l, s, e), h)| (*l, *s, *e, *h)).collect();
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
                 [0, 0, 8, Keyword],
                 [3, 0, 8, CellOdd],
                 [3, 9, 16, CellEven],
                 [3, 17, 24, CellOdd];
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
                 [0, 0, 8, Keyword];
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
                 [0, 0, 8, Keyword],
                 [0, 9, 16, CellOdd],
                 [1, 0, 8, Keyword],
                 [1, 9, 12, CellOdd];
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
                 [0, 0, 8, Keyword],
                 [0, 9, 16, CellOdd];
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
                 [0, 0, 8, Keyword],
                 [0, 9, 16, CellOdd],
                 [1, 0, 8, Keyword],
                 [2, 0, 8, Keyword],
                 [3, 0, 8, Keyword];
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

    h.add_highlight(0, 0, 8, Keyword);
    h.add_highlight(0, 9, 16, CellOdd);
    h.add_highlight(1, 0, 4, Keyword);
    h.add_highlight(1, 5, 12, CellOdd);
    h.add_highlight(1, 13, 20, CellEven);
    h.add_highlight(2, 0, 8, Keyword);
    h.add_highlight(2, 9, 16, CellOdd);

    let v = vec![
      (0, 0, 8, Keyword),
      (0, 9, 16, CellOdd),
      (1, 0, 4, Keyword),
      (1, 5, 12, CellOdd),
      (1, 13, 20, CellEven),
      (2, 0, 8, Keyword),
      (2, 9, 16, CellOdd),
    ];

    // this is not a trivial test, it ascertains the iteration order
    let w: Vec<_> = h.iter().map(|((l, s, e), h)| (*l, *s, *e, *h)).collect();
    assert_eq!(v, w);
  }

}
