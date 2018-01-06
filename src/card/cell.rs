/// Elements of an input line
#[derive(Debug)]
pub enum Cell {
  /// A keyword, given by Card.keyword
  Kw,
  // A fixed, non-keyword entry
  Fixed(&'static str),
  // An integer with a given maximum string-length
  Integer(u8),
  // A float with a given maximum string-length
  Float(u8),
  // A given number of blanks
  Blank(u8),
  // A conctinuation character `&`
  Cont,
  // A string of a given maximum length
  Str(u8),
}
