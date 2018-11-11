//! The highlight module
use std::convert::From;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum HighlightGroup {
  CellEven,
  CellOdd,
  ErrorCellEven,
  ErrorCellOdd,
  Keyword
}

impl From<HighlightGroup> for &'static str {
  fn from(h: HighlightGroup) -> &'static str {
    use self::HighlightGroup::*;

    match h {
      CellEven => "PamCellEven",
      CellOdd => "PamCellOdd",
      ErrorCellEven => "PamErrorCellEven",
      ErrorCellOdd => "PamErrorCellOdd",
      Keyword => "PamKeyword"
    }
  }
}
