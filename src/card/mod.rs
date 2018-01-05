pub mod keyword;
pub mod line;
pub mod cell;
pub mod ges;

use self::line::Line;
use self::keyword::Keyword;
use carddata::*;

#[derive(Debug)]
pub struct Card {
  pub lines: &'static [Line],
  pub ownfold: bool,
}

impl<'a> From<&'a Keyword> for &'static Card {
  fn from(kw: &'a Keyword) -> &'static Card {
      match kw {
        &Keyword::Node => &NODE,
        &Keyword::Cnode => &CNODE,
        &Keyword::Shell => &SHELL,
        &Keyword::Comment => &COMMENT,
        &Keyword::Mass => &MASS,
        &Keyword::Nsmas => &NSMAS,
        &Keyword::Nsmas2 => &NSMAS2
      }
  }
}
