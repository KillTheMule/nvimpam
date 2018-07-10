//! Elements of an input line

/// All the basic elements that can occur on a valid line in a Pamcrash input
/// file, aside from comments and header data.
#[derive(Debug, PartialEq)]
pub enum Cell {
  /// A keyword, given by [Card.keyword](::card::Card.keyword)
  Kw,
  /// A fixed, non-keyword entry
  Fixed(&'static str),
  /// An integer with a given maximum string-length
  Integer(u8),
  /// A float with a given maximum string-length
  Float(u8),
  /// A given number of blanks
  Blank(u8),
  /// A continuation character `&`
  Cont,
  /// A string of a given length
  Str(u8),
  /// A sequence of 0 and 1 of a given length
  Binary(u8),
  /// An alternative of 2 cells
  IntegerorBlank(u8),
}
