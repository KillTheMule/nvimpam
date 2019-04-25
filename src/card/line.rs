//! An enum to classify the several types of lines that can occur inside a card
//! of a Pamcrash input file. Might not really be a line (see
//! [GES](crate::card::line::Line::Ges), rather than zero or more lines.
use std::{cmp, ops::Range};

use atoi::atoi;

use crate::card::{cell::Cell, cell::CellHint, ges::GesType, keyword::Keyword};

/// A line (actually, zero or more lines) inside a card in a Pamcrash input
/// file.
#[derive(Debug, PartialEq)]
pub enum Line {
  /// A standard line, containing several cells of a fixed width
  Cells(&'static [Cell]),
  /// A [General Entity Selection](crate::card::ges::GesType), can consist of
  /// several lines
  Ges(GesType),
  /// A line that provides a Conditional
  Provides(&'static [Cell], Conditional),
  /// An optional line, depending on a conditional
  ///
  /// The conditional is given as an index. Walking through the lines of a
  /// card will push the result of all conditionals to a Vec in order of
  /// occurence. The optional lines will index directly into the Vec to check
  /// for the result of their conditional.
  Optional(&'static [Cell], u8),
  /// A line that is repeated
  ///
  /// The [number of repeats](crate::card::line::CondResult::Number) is given
  /// as an index, see the doc for
  /// [`Optional`](crate::card::line::Line::Optional)
  Repeat(&'static [Cell], u8),
  /// A block of lines, ended by a line starting with the given string.
  Block(&'static [Line], &'static [u8]),
  /// A block that's entirely optional, starting with a line of a given string
  /// and ending in a line with another given string
  OptionalBlock(&'static [u8], &'static [u8]),
}

impl Line {
  /// If the line starts with a [`Keyword`](crate::card::keyword::Keyword),
  /// return it. Otherwise, return `None`.
  #[inline]
  pub fn keyword(&self) -> Option<Keyword> {
    use self::Line::*;

    match *self {
      Cells(s) | Provides(s, _) => s[0].keyword(),
      _ => None,
    }
  }

  /// If the line contains a slice of [`Cells`](crate::card::cell::Cell), return
  /// it. Otherwise, return None.
  #[inline]
  pub fn cells(&self) -> Option<&'static [Cell]> {
    use self::Line::*;

    match *self {
      Cells(s) | Provides(s, _) | Optional(s, _) | Repeat(s, _) => Some(s),
      Ges(_) | Block(_, _) | OptionalBlock(_, _) => None,
    }
  }

  /*
  /// Return an iterator over the highlight of a line. The `text` is
  /// parsed in the process to potentially highlight errors.
  pub fn highlights<'a>(&'a self, text: &'a [u8]) -> HlIter<'a> {
    HlLine {
      cardline: &self,
      text,
    }
    .into_iter()
  }
  */
}

/// An enum to represent different conditionals on lines
#[derive(Debug, PartialEq)]
pub enum Conditional {
  /// The byte (2nd arg) at the given index (1st arg, 0-based!) is the given
  /// one.
  RelChar(u8, u8),
  // The integer at the cell given by the range is the second number
  Int(Range<u8>, u8),
  // Read a number from a given cell
  Number(Range<u8>),
}

/// An enum to represent the different results of conditionals
#[derive(Debug, PartialEq)]
pub enum CondResult {
  Bool(bool),
  Number(Option<usize>),
}

impl Conditional {
  /// Given a line, evaluate the conditional on it
  pub fn evaluate(&self, line: &[u8]) -> CondResult {
    use self::CondResult::*;

    match *self {
      Conditional::RelChar(idx, c) => Bool(line.get(idx as usize) == Some(&c)),
      Conditional::Int(ref r, b) => {
        let range = r.start as usize..cmp::min(line.len(), r.end as usize);

        let cell = match line.get(range) {
          Some(c) => c,
          None => return Bool(false),
        };

        let firstdigit = cell
          .iter()
          .position(|b| *b >= b'0' && *b <= b'9')
          .unwrap_or(0_usize);

        Bool(
          cell
            .get(firstdigit..)
            .map_or(false, |s| atoi::<usize>(s) == Some(b as usize)),
        )
      }
      Conditional::Number(ref r) => {
        let range = r.start as usize..cmp::min(line.len(), r.end as usize);

        let cell = match line.get(range) {
          Some(c) => c,
          None => return Bool(false),
        };

        let firstdigit = cell
          .iter()
          .position(|b| *b >= b'0' && *b <= b'9')
          .unwrap_or(0_usize);

        Number(cell.get(firstdigit..).and_then(|s| atoi::<usize>(s)))
      }
    }
  }
}

pub struct LineHint {
  pub cellhints: &'static [CellHint],
}

impl LineHint {
  pub fn get_cell(&self, column: u8) -> Option<&'static CellHint> {
    let mut sum = 0;

    for cellhint in self.cellhints.iter() {
      sum += cellhint.len();
      if sum > column {
        return Some(cellhint);
      }
    }

    None
  }
}

#[cfg(test)]
mod tests {
  use crate::card::line::{CondResult::*, Conditional};

  #[test]
  fn relchar_can_be_evaluated() {
    let cond1 = Conditional::RelChar(2, b'b');
    let cond2 = Conditional::RelChar(3, b'b');
    let line = "abbxy oaslkj";

    assert_eq!(Bool(true), cond1.evaluate(line.as_ref()));
    assert_eq!(Bool(false), cond2.evaluate(line.as_ref()));
  }

  #[test]
  fn relchar_out_of_bounds() {
    let cond1 = Conditional::RelChar(95, b'b');
    let line = "abbxy oaslkj";

    assert_eq!(Bool(false), cond1.evaluate(line.as_ref()));
  }

}
