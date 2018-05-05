//! This modules holds the the global static element [`Card`](::card::Card)
//! instances.
use card::cell::Cell::*;
use card::keyword::Keyword::*;
use card::line::Line::*;
use card::Card;

pub static SHELL: Card = Card {
  lines: &[Cells(&[
    Kw,
    Integer(8),
    Integer(8),
    Integer(8),
    Integer(8),
    Integer(8),
    Integer(8),
    Blank(8),
    Float(8),
  ])],
  ownfold: false,
  keyword: Shell,
};
