//! Elements of an input line

/// All the basic elements that can occur on a valid line in a Pamcrash
/// input file, aside from comments and header data.
use std::{convert::From, str};

use lexical::FromBytesLossy;

use crate::card::{hint::Hint, keyword::Keyword};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FixedStr {
  Name,
  Weight,
  Rmat,
  EndPart, // 'END_PART'
  Comment, // '#'
}

impl FixedStr {
  pub(crate) fn len(self) -> u8 {
    use self::FixedStr::*;
    match self {
      Name | Rmat => 4,
      Weight => 6,
      EndPart => 8, // 'END_PART'
      Comment => 1, // '#'
    }
  }
}

impl From<FixedStr> for &'static str {
  fn from(f: FixedStr) -> Self {
    use self::FixedStr::*;
    match f {
      Name => "NAME",
      Weight => "WEIGHT",
      Rmat => "RMAT",
      EndPart => "END_PART",
      Comment => "#",
    }
  }
}

#[derive(Debug, PartialEq)]
pub enum Cell {
  /// A [`keyword`](crate::card::keyword::Keyword)
  Kw(Keyword),
  /// A fixed, non-keyword entry
  Fixed(FixedStr),
  /// An integer with a given maximum string-length
  Integer(u8, Hint),
  /// A float with a given maximum string-length
  Float(u8, Hint),
  /// A given number of blanks
  Blank(u8),
  /// A continuation character `&`
  Cont,
  /// A string of a given length
  Str(u8, Hint),
  /// A sequence of 0 and 1 of a given length
  Binary(u8, Hint),
  /// An alternative of 2 cells
  IntegerorBlank(u8, Hint),
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
    use crate::card::cell::Cell::*;
    match *self {
      Kw(k) => k.len(),
      Fixed(ref s) => {
        debug_assert!(s.len() < 81);
        s.len() as u8
      }
      Cont => 1,
      Integer(u, _)
      | Float(u, _)
      | Blank(u)
      | Str(u, _)
      | Binary(u, _)
      | IntegerorBlank(u, _) => u,
    }
  }

  #[inline]
  pub fn is_empty(&self) -> bool {
    use crate::card::cell::Cell::*;
    match *self {
      Integer(u, _)
      | Float(u, _)
      | Blank(u)
      | Str(u, _)
      | Binary(u, _)
      | IntegerorBlank(u, _) => u == 0,
      _ => false,
    }
  }

  /// Checks if the contents of the cell in the file are valid for the type of
  /// the cell. Right now, only checks [`Float`](crate::card::cell::Cell::Float)
  /// cells. Returns `false` if the slice is empty.
  ///
  /// TODO(KillTheMule): Extend. Implement Pyvars.
  #[inline]
  pub fn verify(&self, s: &[u8]) -> bool {
    use self::Cell::*;

    match *self {
      Float(_, _) => {
        if s.is_empty() {
          return false;
        }

        let trimmed = self.trim(s);

        trimmed.is_empty()
          || f64::try_from_bytes_lossy(&trimmed).is_ok()
          || (trimmed.first() == Some(&b'<') && trimmed.last() == Some(&b'>'))
      }
      _ => true,
    }
  }

  /// Returns the byte array with leading/trailing b' ' removed
  #[inline]
  pub fn trim<'a, 'b>(&'a self, s: &'b [u8]) -> &'b [u8] {
    if s.is_empty() {
      return s;
    }

    let mut i = 0;
    let mut j = s.len() - 1;

    // Safe, because 0 <= i < s.len() - 1
    while i < j && unsafe { s.get_unchecked(i) == &b' ' } {
      i += 1;
    }

    // Safe, because s.len() - 1 >= j > 0
    while j > i && unsafe { s.get_unchecked(j) == &b' ' } {
      j -= 1;
    }

    if j > i {
      // Safe, see comments above
      unsafe { s.get_unchecked(i..=j) }
    } else {
      // j = i
      let last = unsafe { s.get_unchecked(i..=j) };
      if last == b" " {
        &[]
      } else {
        last
      }
    }
  }

  pub fn hint(&self) -> &'static str {
    use self::Cell::*;
    match *self {
      Kw(_) => "Keyword",
      Fixed(_) => "Fixed string",
      Integer(_, h)
      | Float(_, h)
      | Str(_, h)
      | Binary(_, h)
      | IntegerorBlank(_, h) => h.into(),
      Blank(_) => "Blanks",
      Cont => "&",
    }
  }
}

#[cfg(test)]
mod tests {
  use super::Cell;
  use crate::card::hint::Hint;

  #[test]
  fn verifying_floats() {
    let cell = Cell::Float(10, Hint::X);

    assert!(!cell.verify("".as_ref()));
    assert!(cell.verify("  ".as_ref()));
    assert!(cell.verify("1e5".as_ref()));
    assert!(cell.verify(" 1e5".as_ref()));
    assert!(cell.verify(" 1e5 ".as_ref()));
    assert!(cell.verify("1e5 ".as_ref()));
    assert!(cell.verify("1e-5".as_ref()));
    assert!(cell.verify(" 1e-5".as_ref()));
    assert!(cell.verify(" 1e-5 ".as_ref()));
    assert!(cell.verify("1e-5 ".as_ref()));
    assert!(cell.verify("1e-05".as_ref()));
    assert!(cell.verify(" 1e-05".as_ref()));
    assert!(cell.verify(" 1e-05 ".as_ref()));
    assert!(cell.verify("1e-05 ".as_ref()));
    assert!(cell.verify("1.5".as_ref()));
    assert!(cell.verify(" 1.5".as_ref()));
    assert!(cell.verify(" 1.5 ".as_ref()));
    assert!(cell.verify("1.5 ".as_ref()));
    assert!(cell.verify("1.".as_ref()));
    assert!(cell.verify(" 1.".as_ref()));
    assert!(cell.verify(" 1. ".as_ref()));
    assert!(cell.verify("1. ".as_ref()));
    assert!(cell.verify("1".as_ref()));
    assert!(cell.verify(" 1".as_ref()));
    assert!(cell.verify(" 1 ".as_ref()));
    assert!(cell.verify("1 ".as_ref()));
    assert!(!cell.verify("x".as_ref()));
    assert!(!cell.verify(" x".as_ref()));
    assert!(!cell.verify(" x ".as_ref()));
    assert!(!cell.verify("x ".as_ref()));
    assert!(!cell.verify("1.x".as_ref()));
    assert!(!cell.verify(" 1.x".as_ref()));
    assert!(!cell.verify(" 1.x ".as_ref()));
    assert!(!cell.verify("1.x ".as_ref()));
    assert!(cell.verify("<var>".as_ref()));
    assert!(cell.verify(" <var>".as_ref()));
    assert!(cell.verify(" <var> ".as_ref()));
    assert!(cell.verify("<var> ".as_ref()));
    assert!(cell.verify("<var >".as_ref()));
  }

}
