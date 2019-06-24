//! A datastructure to represent a `Card` of the FEM-Solver Pamcrash.
//!
//! There is a 1-1 correspondence between a [`Card`](crate::card::Card) and
//! a [`Keyword`](crate::card::keyword::Keyword), but they are used differently.
//! A [`Keyword`](crate::card::keyword::Keyword) is used as a parsing result for
//! strings all over the place, while [`Card`](crate::card::Card) is used to
//! define global static values the represent the concrete CARD entities of
//! Pamcrash.
//!
//! The definitions of the global static values can be found in the
//! [`carddata`](crate::carddata) module.
pub mod cell;
pub mod ges;
pub mod hint;
pub mod keyword;
pub mod line;

use self::{keyword::Keyword, line::Line};
use crate::carddata::*;

/// A card consists of severals [`Line`](crate::card::line::Line). If `ownfold`
/// is true, than each card of this type will get an own fold. Otherwise, all
/// adjacent cards of that type are gathered into one fold.
#[derive(Debug, PartialEq)]
pub struct Card {
  pub lines: &'static [Line],
  pub ownfold: bool,
}

impl Card {
  /// Return the keyword of the first line of the card. This is the only keyword
  /// in the card, and will always be present.
  #[inline]
  pub fn keyword(&self) -> Keyword {
    self.lines[0].keyword().unwrap_or_else(|| {
      panic!(format!(
        "This card did not have a keyword starting its first line: {:?}",
        self
      ))
    })
  }
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
      /*
      // Link
      Keyword::Elink => &ELINK,
      Keyword::Llink => &LLINK,
      Keyword::Slink => &SLINK,
      Keyword::Plink => &PLINK,
      Keyword::Tied => &TIED,
      // Part 3D
      Keyword::PartSolid => &PARTSOLID,
      Keyword::PartBshel => &PARTBSHEL,
      Keyword::PartTetra => &PARTTETRA,
      Keyword::PartSphel => &PARTSPHEL,
      Keyword::PartCos3d => &PARTCOS3D,
      // Part 2D
      Keyword::PartTshel => &PARTTSHEL,
      Keyword::PartShell => &PARTSHELL,
      Keyword::PartMembr => &PARTMEMBR,
      // Part 1D
      Keyword::PartBar => &PARTBAR,
      Keyword::PartBeam => &PARTBEAM,
      Keyword::PartSpring => &PARTSPRING,
      Keyword::PartSprgbm => &PARTSPRGBM,
      Keyword::PartMbspr => &PARTMBSPR,
      Keyword::PartJoint => &PARTJOINT,
      Keyword::PartKjoin => &PARTKJOIN,
      Keyword::PartMbkjn => &PARTMBKJN,
      Keyword::PartMtojnt => &PARTMTOJNT,
      Keyword::PartTied => &PARTTIED,
      Keyword::PartSlink => &PARTSLINK,
      Keyword::PartElink => &PARTELINK,
      Keyword::PartLlink => &PARTLLINK,
      Keyword::PartPlink => &PARTPLINK,
      Keyword::PartGap => &PARTGAP,
      */
      // Constraint
      Keyword::Mtoco => &MTOCO,
      Keyword::Otmco => &OTMCO,
      Keyword::Rbody0 => &RBODY0,
      Keyword::Rbody1 => &RBODY1,
      Keyword::Rbody2 => &RBODY2,
      Keyword::Rbody3 => &RBODY3,
      /*
      // Auxiliaries
      Keyword::Group => &GROUP,
      */
    }
  }
}
