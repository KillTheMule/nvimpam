//! Elements of an input line

/// All the basic elements that can occur on a valid line in a Pamcrash
/// input file, aside from comments and header data.
use std::str;

use card::keyword::Keyword;

#[derive(Debug, PartialEq)]
pub enum Cell {
  /// A [`keyword`](::card::keyword::Keyword)
  Kw(Keyword),
  /// A fixed, non-keyword entry
  Fixed(&'static str),
  /// An integer with a given maximum string-length
  Integer(u8),
  /// A float with a given maximum string-length
  Float(u8),
  /// A given number of blanks
  Blank(u8),
  /// A continuation character `&`
  Cont,
  /// A string of a given length
  Str(u8),
  /// A sequence of 0 and 1 of a given length
  Binary(u8),
  /// An alternative of 2 cells
  IntegerorBlank(u8),
}

impl Cell {
  #[inline]
  pub fn keyword(&self) -> Option<Keyword> {
    match *self {
      Cell::Kw(k) => Some(k),
      _ => None,
    }
  }

  #[inline]
  pub fn len(&self) -> u8 {
    use card::cell::Cell::*;
    match *self {
      Kw(k) => k.len(),
      Fixed(s) => {
        debug_assert!(s.len() < 81);
        s.len() as u8
      }
      Cont => 1,
      Integer(u) | Float(u) | Blank(u) | Str(u) | Binary(u)
      | IntegerorBlank(u) => u,
    }
  }

  #[inline]
  pub fn is_empty(&self) -> bool {
    use card::cell::Cell::*;
    match *self {
      Integer(u) | Float(u) | Blank(u) | Str(u) | Binary(u)
      | IntegerorBlank(u) => u == 0,
      _ => false,
    }
  }

  #[inline]
  pub fn verify(&self, s: &[u8]) -> bool {
    use self::Cell::*;

    match *self {
      Float(_) => {
        s.iter().all(|x| *x == b' ')
          || str::from_utf8(s)
            .map(|l| l.trim_matches(' ').parse::<f64>().is_ok())
            == Ok(true)
      }
      _ => true,
    }
  }
}
