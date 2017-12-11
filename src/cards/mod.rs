//! This module provides the [Card](cards/enum.Card.html) enum to classify lines
//! according to what card type they belong to. The term "Card" is from the
//! FEM solver Pamcrash, but generally used among FEM solvers.

/// An enum to denote the several types of cards a line might belong to. For now
/// carries only information equivalent to the keyword, not the subtypes, e.g.
/// CNTAC types 33 and 36 will both be denoted by type Cntac
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Card {
  Node,
  Shell,
  Comment,
}

impl Card {
  /// Parse a string to determine if it starts with the keyword of a card.
  #[inline]
  pub fn parse_str<'a>(s: &'a str) -> Option<Card> {
    use self::Card::*;
    if s.starts_with("NODE") {
      return Some(Node);
    } else if s.starts_with("SHELL") {
      return Some(Shell);
    } else if s.starts_with("$") || s.starts_with("#") {
      return Some(Comment);
    } else {
      return None;
    };
  }
}
