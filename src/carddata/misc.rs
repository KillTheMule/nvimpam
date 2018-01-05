use card::Card;
use card::cell::Cell::*;
use card::line::Line::*;

pub static COMMENT: Card = Card {
  lines: &[Cells(&[Fixed("#")])],
  ownfold: false,
};
