use card::cell::Cell::*;
use card::keyword::Keyword::*;
use card::line::Line::*;
use card::Card;

pub static COMMENT: Card = Card {
  lines: &[Cells(&[Fixed("#")])],
  ownfold: false,
  keyword: Comment,
};
