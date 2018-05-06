//! A datastructure to represent a `Card` of the FEM-Solver Pamcrash.
//!
//! There is a 1-1 correspondence between a [`Card`](Card) and
//! a [`Keyword`](Keyword), but are used differently. A
//! [`Keyword`](keyword) is used as a parsing result for
//! strings all over the place, while [`Card`](Card) is used to
//! define global static values the represent the concrete CARD entities of
//! Pamcrash.
//!
//! The definitions of the global static values can be found in the
//! [`carddata`](::carddata) module.
//!
pub mod cell;
pub mod ges;
pub mod keyword;
pub mod line;

use self::keyword::Keyword;
use self::line::Line;
use carddata::*;

/// A card consists of severals [`Line`](::card::line::Line)s, and starts with a
/// given [`Keyword`](::card::keyword::Keyword). If `ownfold` is true, than each
/// card of this type will get an own fold. Otherwise, all adjacent cards of
/// that types are gathered into one fold.
#[derive(Debug)]
pub struct Card {
  pub lines: &'static [Line],
  pub ownfold: bool,
  pub keyword: Keyword,
}

impl<'a> From<&'a Keyword> for &'static Card {
  fn from(kw: &'a Keyword) -> &'static Card {
    match *kw {
      // Node
      Keyword::Node => &NODE,
      Keyword::Cnode => &CNODE,
      Keyword::Mass => &MASS,
      Keyword::Nsmas => &NSMAS,
      Keyword::Nsmas2 => &NSMAS2,
      // Element
      Keyword::Solid => &SOLID,
      Keyword::Hexa20 => &HEXA20,
      Keyword::Pent15 => &PENT15,
      Keyword::Penta6 => &PENTA6,
      Keyword::Tetr10 => &TETR10,
      Keyword::Tetr4 => &TETR4,
      Keyword::Bshel => &BSHEL,
      Keyword::Tshel => &TSHEL,
      Keyword::Shell => &SHELL,
      Keyword::Shel6 => &SHEL6,
      Keyword::Shel8 => &SHEL8,
      Keyword::Membr => &MEMBR,
      Keyword::Beam => &BEAM,
      Keyword::Sprgbm => &SPRGBM,
      Keyword::Bar => &BAR,
      Keyword::Spring => &SPRING,
      Keyword::Joint => &JOINT,
      Keyword::Kjoin => &KJOIN,
      Keyword::Mtojnt => &MTOJNT,
      Keyword::Sphel => &SPHEL,
      Keyword::Sphelo => &SPHELO,
      Keyword::Gap => &GAP,
      Keyword::Impma => &IMPMA,
      // Link
      Keyword::Elink => &ELINK,
    }
  }
}
