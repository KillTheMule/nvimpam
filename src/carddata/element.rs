use card::keyword::Keyword::*;
use card::Card;
use card::cell::Cell::*;
use card::line::Line::*;

pub static SHELL: Card = Card {
  lines: &[
    Cells(
      &[
        Kw,
        Integer(8),
        Integer(8),
        Integer(8),
        Integer(8),
        Integer(8),
        Integer(8),
        Blank(8),
        Float(8),
      ],
    ),
  ],
  ownfold: false,
  keyword: Shell,
};
