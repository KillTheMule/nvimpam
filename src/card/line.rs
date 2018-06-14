//! An enum to classify the several types of lines that can occur inside a card
//! of a Pamcrash input file. Might not really be a line (see GES).
use std::ops::Range;
use std::cmp;

use card::cell::Cell;
use card::ges::GesType;

/// A line inside a card in a Pamcrash input file.
#[derive(Debug, PartialEq)]
pub enum Line {
  /// A standard line, containing several cells of a fixed width
  Cells(&'static [Cell]),
  /// A General Entity Selection, can consist of several lines
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
  /// The [number of repeats](::card::line::CondResult) is given as an index,
  /// see the doc for [`Optional`](::card::line::Line)
  Repeat(&'static [Cell], u8),
  /// A block of lines, ended by a line starting with the given string.
  Block(&'static [Line], &'static str),
  /// A black that's entirely optional, starting with a line of a given string
  /// and ending in a line with another keywordgiven string
  OptionalBlock(&'static str, &'static str)
}

/// An enum to represent different conditionals on lines
#[derive(Debug, PartialEq)]
pub enum Conditional {
  /// The char at the given index (0-based!) is the given one.
  RelChar(u8, char),
  // The integer at the cell given by the first number is the second number
  Int(Range<usize>, u8),
  // Read a number from a given cell
  Number(Range<usize>),
}

// An enum to represent the different results of conditionals
#[derive(Debug, PartialEq)]
pub enum CondResult {
  Bool(bool),
  Number(Option<usize>),
}

impl Conditional {
  /// Given a line, evaluate the condition on it
  pub fn evaluate<'a, T: 'a>(&self, line: &'a T) -> CondResult
  where
    T: AsRef<str>,
  {
    use self::CondResult::*;

    match *self {
      Conditional::RelChar(idx, c) => {
        let idx = idx as usize;
        Bool(line.as_ref().get(idx..idx + 1) == Some(&c.to_string()))
      }
      Conditional::Int(ref r, b) => {
        let lineref = line.as_ref();
        let linelen = lineref.len();
        let lower = r.start;
        let upper = r.end;
        let new_upper = cmp::min(linelen, upper);
        let range = lower..new_upper;

        let cell = lineref.get(range);

        match cell {
          None => Bool(false),
          Some(s) => match s.trim().parse::<u8>() {
            Ok(x) if x == b => Bool(true),
            _ => Bool(false),
          },
        }
      }
      Conditional::Number(ref r) => {
        let s = line.as_ref();
        let l = s.len();
        let lower = r.start;
        let upper = r.end;
        let new_upper = cmp::min(l, upper);
        let range = lower..new_upper;

        let cell = s.get(range);

        match cell {
          None => Number(None),
          Some(s) => match s.trim().parse::<usize>() {
              Ok(x) => Number(Some(x)),
              _ => Number(None),
            }
          }
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
    let cond1 = Conditional::RelChar(2, 'b');
    let cond2 = Conditional::RelChar(3, 'b');
    let line = "abbxy oaslkj";

    assert_eq!(Bool(true), cond1.evaluate(&line));
    assert_eq!(Bool(false), cond2.evaluate(&line));
  }

  #[test]
  fn relchar_out_of_bounds() {
    let cond1 = Conditional::RelChar(95, 'b');
    let line = "abbxy oaslkj";

    assert_eq!(Bool(false), cond1.evaluate(&line));
  }

}
