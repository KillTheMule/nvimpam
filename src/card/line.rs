//! An enum to classify the several types of lines that can occur inside a card
//! of a Pamcrash input file. Might not really be a line (see
//! [GES](::card::line::Line::Ges).
use std::cmp;
use std::ops::Range;

use atoi::atoi;

use card::cell::Cell;
use card::ges::GesType;

/// A line inside a card in a Pamcrash input file.
#[derive(Debug, PartialEq)]
pub enum Line {
  /// A standard line, containing several cells of a fixed width
  Cells(&'static [Cell]),
  /// A [General Entity Selection](::card::ges::GesType), can consist of
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
  /// The [number of repeats](::card::line::CondResult::Number) is given as an
  /// index, see the doc for [`Optional`](::card::line::Line::Optional)
  Repeat(&'static [Cell], u8),
  /// A block of lines, ended by a line starting with the given string.
  Block(&'static [Line], &'static [u8]),
  /// A block that's entirely optional, starting with a line of a given string
  /// and ending in a line with another given string
  OptionalBlock(&'static [u8], &'static [u8]),
}

/// An enum to represent different conditionals on lines
#[derive(Debug, PartialEq)]
pub enum Conditional {
  /// The byte (2nd arg) at the given index (1st arg) (0-based!) is the given
  /// one.
  RelChar(u8, u8),
  // The integer at the cell given by the range is the second number
  Int(Range<u8>, u8),
  // Read a number from a given cell
  Number(Range<u8>),
}

// An enum to represent the different results of conditionals
#[derive(Debug, PartialEq)]
pub enum CondResult {
  Bool(bool),
  Number(Option<usize>),
}

impl Conditional {
  /// Given a line, evaluate the conditional on it
  pub fn evaluate(&self, line: &[u8]) -> CondResult
  {
    use self::CondResult::*;

    match *self {
      Conditional::RelChar(idx, c) => {
        let idx = idx as usize;
        Bool(line.get(idx) == Some(&c))
      }
      Conditional::Int(ref r, b) => {
        let range =
          r.start as usize..cmp::min(line.len(), r.end as usize);

        let cell = match line.get(range) {
          Some(c) => c,
          None => return Bool(false)
        };

        let firstdigit = cell
            .iter()
            .position(|b| *b >= b'0' && *b <= b'9')
            .unwrap_or(0usize);

        Bool(
          cell.get(firstdigit..)
            .map(|s| atoi::<usize>(s) == Some(b as usize))
            .unwrap_or(false),
        )
      }
      Conditional::Number(ref r) => {
        let range =
          r.start as usize..cmp::min(line.len(), r.end as usize);

        let cell = match line.get(range) {
          Some(c) => c,
          None => return Bool(false)
        };

        let firstdigit = cell
            .iter()
            .position(|b| *b >= b'0' && *b <= b'9')
            .unwrap_or(0usize);

        Number(
          cell.get(firstdigit..)
            .map(|s| atoi::<usize>(s))
            .unwrap_or(None)
        )
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use card::line::CondResult::*;
  use card::line::Conditional;

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
