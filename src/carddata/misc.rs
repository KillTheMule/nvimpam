use card::Card;
use card::keyword::Keyword::*;
use card::cell::Cell::*;
use card::line::Line::*;

pub static COMMENT: Card = Card {
  lines: &[Cells(&[Fixed("#")])],
  ownfold: false,
  keyword: Comment,
};
