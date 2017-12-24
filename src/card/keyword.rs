//! This module provides the [Keyword](cards/enum.Keyword.html) enum to
//! classify lines
//! according to what card type they belong to. The term "Keyword" is from the
//! FEM solver Pamcrash, but generally used among FEM solvers.

/// An enum to denote the several types of cards a line might belong to. For now
/// carries only information equivalent to the keyword, not the subtypes, e.g.
/// CNTAC types 33 and 36 will both be denoted by type Cntac
use std::iter;
use std::slice;

use card::Card;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Keyword {
  Node,
  Cnode,
  Shell,
  Comment,
  Mass,
  Nsmas,
  Nsmas2,
}

impl Keyword {
  /// Parse a string to determine if it starts with the keyword of a card.
  #[inline]
  pub fn parse<T: AsRef<str>>(s: T) -> Option<Keyword> {
    use self::Keyword::*;

    let s = s.as_ref();

    if s.starts_with("$") || s.starts_with("#") {
      return Some(Comment);
    } else if s.starts_with("NODE  / ") {
      return Some(Node);
    } else if s.starts_with("CNODE / ") {
      return Some(Cnode);
    } else if s.starts_with("MASS  / ") {
      return Some(Mass);
    } else if s.starts_with("NSMAS / ") {
      return Some(Nsmas);
    } else if s.starts_with("NSMAS2/ ") {
      return Some(Nsmas2);
    } else if s.starts_with("SHELL / ") {
      return Some(Shell);
    } else {
      return None;
    };
  }

  /// Get the end of the type that we found on the last line the iterator
  /// returned.
  ///
  /// Returns the index of the last line of the fold (`None` if there is no
  /// such, i.e. the file ended or another keyword was found early), the
  /// Keyword of the last line the iterator has returned (None if it does not
  /// have a keyword), and the index of the last line the  iterator has
  /// returned (this will only be none if we exhausted the file, i.e. the
  /// iterator returned `None`). The last two are necessary because we
  /// advanced the iterator 1 line further to look at the following line, and
  /// there might be comment lines between the end of the fold in the last
  /// line we looked into. Those comment lines will not be folded, but we
  /// advanced through them anyways to check for the next non-comment line.
  #[inline]
  pub fn get_foldend<'a, T: AsRef<str>>(
    &self,
    it: &mut iter::Enumerate<slice::Iter<'a, T>>,
  ) -> (Option<u64>, Option<Keyword>, Option<u64>) {
    let card: &Card = self.into();

    if card.ownfold {
      let num = card.lines.len();
      let mut i = 0;
      let mut idx = 0;
      let mut line;


      while i < num {
        let tmp = it.next();
        match tmp {
          None => return (None, None, None),
          Some((j, l)) => {
            idx = j;
            line = l;
          }
        }

        if let Some(k) = Keyword::parse(line) {
          if k == Keyword::Comment {
            i += 1;
            continue;
          }
          return (None, Some(k), Some(idx as u64));
        } else {
          i += 1;
        }
      }

      let tmp = it.next();
      match tmp {
        None => return (Some(idx as u64), None, None),
        Some((i, l)) => {
          return (Some(idx as u64), Keyword::parse(l), Some(i as u64))
        }
      }
    } else {
      // !card.ownfold
      let mut idx;
      let mut line;
      let mut curkw;
      let mut idx_before_comment = 0;

      let tmp = it.next();
      match tmp {
        None => return (None, None, None),
        Some((j, l)) => {
          idx = j;
          line = l;
          curkw = Keyword::parse(line);
        }
      }

      if curkw.is_none() {
        return (None, None, None);
      }

      while curkw == Some(*self) || curkw == Some(Keyword::Comment) {
        if curkw == Some(Keyword::Comment) && idx_before_comment == 0 {
          idx_before_comment = idx - 1;
        } else if curkw == Some(*self) && idx_before_comment > 0 {
          idx_before_comment = 0;
        }
        let tmp = it.next();
        match tmp {
          None => return (Some(idx as u64), None, None),
          Some((j, l)) => {
            idx = j;
            line = l;
            curkw = Keyword::parse(line);
          }
        }
      }

      if idx_before_comment > 0 {
        return (Some(idx_before_comment as u64), curkw, Some(idx as u64));
      } else {
        return (Some(idx as u64 - 1), curkw, Some(idx as u64));
      }
    }
  }
}
