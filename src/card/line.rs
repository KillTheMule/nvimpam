use card::cell::Cell;
use card::ges::GesType;

#[derive(Debug)]
pub enum Line {
  Cells(&'static [Cell]),
  Ges(GesType),
  Optional(&'static [Cell], Option<Conditional>)
}

#[derive(Debug)]
pub enum Conditional {
  RelChar(i64, u8, char)
}
