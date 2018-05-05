//! This module provides the [Keyword](cards/enum.Keyword.html) enum to
//! classify lines
//! according to what card type they belong to. The term "Keyword" is from the
//! FEM solver Pamcrash, but generally used among FEM solvers.

/// An enum to denote the several types of cards a line might belong to. For now
/// carries only information equivalent to the keyword, not the subtypes, e.g.
/// CNTAC types 33 and 36 will both be denoted by type Cntac
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Keyword {
  Node,
  Cnode,
  Shell,
  Mass,
  Nsmas,
  Nsmas2,
}

impl Keyword {
  /// Parse a string to determine if it starts with the keyword of a card.
  #[inline]
  pub fn parse<T: AsRef<str>>(s: &T) -> Option<Keyword> {
    use self::Keyword::*;

    let s = s.as_ref().as_bytes();
    let len = s.len();

    if len == 0 || len < 8 {
      None
    } else {
      let start = &s[0..8];

      match start {
        b"NODE  / " => Some(Node),
        b"CNODE / " => Some(Cnode),
        b"MASS  / " => Some(Mass),
        b"NSMAS / " => Some(Nsmas),
        b"NSMAS2/ " => Some(Nsmas2),
        b"SHELL / " => Some(Shell),
        _ => None,
      }
    }
  }
}
