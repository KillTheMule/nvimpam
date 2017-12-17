use cards::cells::Cell;

#[derive(Debug)]
pub struct Card {
  pub lines: &'static [Line]
}

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

#[derive(Debug)]
pub enum GesType {
  GesNode,
  GesEle
}
