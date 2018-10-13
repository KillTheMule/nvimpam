//! This module holds the datastructure for the Lines of the buffer. For now,
//! it's simply a `Vec<String>` with an appropriate API.
//!
//! Future ideas, if performance isn't enough: Skip list, gap buffer (adapted to
//! lines instead of strings), rope (adapted to lines instead of strings)
use std::convert::{AsRef, From};
use std::fs::File;
use std::io::Read;
use std::ops::Deref;
use std::path::Path;

use failure::Error;
use failure::ResultExt;

/// An enum representing the line of a file, either as a byte slice (which we
/// obtain from reading a file into a `Vec<u8>` and splitting on newlines) or an
/// owned `String` (which we get from neovim's buffer update API).
#[derive(Debug, PartialEq)]
pub enum Line<'a> {
  OriginalLine(&'a [u8]),
  ChangedLine(String),
}

impl<'a> AsRef<[u8]> for Line<'a> {
  fn as_ref(&self) -> &[u8] {
    match self {
      Line::OriginalLine(l) => l,
      Line::ChangedLine(s) => s.as_ref(),
    }
  }
}

/// The struct to hold the lines.
#[derive(Debug, Default)]
pub struct Lines<'a>(Vec<Line<'a>>);

impl<'a> Lines<'a> {
  /// Create a new empty `Lines` struct
  pub fn new() -> Lines<'a> {
    Lines(Vec::new())
  }

  /// Returns the number of lines
  pub fn len(&self) -> usize {
    self.0.len()
  }

  /// Check if there are any lines
  pub fn is_empty(&self) -> bool {
    self.0.is_empty()
  }

  /// Create a new `Lines` struct from a `Vec<String>`
  pub fn from(v: Vec<String>) -> Lines<'a> {
    let w = v.into_iter().map(Line::ChangedLine).collect();
    Lines(w)
  }

  /// Create a new `Lines` struct from a byte slice by splitting on newlines.
  pub fn from_slice(v: &'a [u8]) -> Lines<'a> {
    let w: Vec<Line> = v
      .split(|b| *b == b'\n')
      .map(|l| Line::OriginalLine(l))
      .collect();
    Lines(w)
  }

  /// Read a file into a `Vec<u8>`. For usage with `::lines::Lines::from_slice`.
  pub fn read_file<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, Error> {
    let mut file = File::open(&path).with_context(|e| {
      format!("Error opening {}: {}", path.as_ref().display(), e)
    })?;
    let l = file.metadata().unwrap().len();
    let mut v: Vec<u8> = Vec::with_capacity(l as usize);
    let _ = file.read_to_end(&mut v).with_context(|e| {
      format!("Error reading {}: {}", path.as_ref().display(), e)
    })?;

    Ok(v)
  }

  /// Update Lines:
  ///   * `first` and `last` are zero-indexed (just as Lines itself)
  ///   * `last` is the first line that has _not_ been updated
  pub fn update(&mut self, first: usize, last: usize, linedata: Vec<String>) {
    let range = first..last;
    let _v = self
      .0
      .splice(range, linedata.into_iter().map(Line::ChangedLine));
  }
}

impl<'a> Deref for Lines<'a> {
  type Target = [Line<'a>];

  fn deref(&self) -> &[Line<'a>] {
    &self.0
  }
}

impl<'a> From<Vec<String>> for Lines<'a> {
  fn from(v: Vec<String>) -> Lines<'a> {
    Lines(v.into_iter().map(Line::ChangedLine).collect())
  }
}

#[cfg(test)]
mod tests {
  use lines::Line::*;
  use lines::Lines;

  const LINES: &str = "This\nis \nan \nexample \nof \nsome \nlines \n.";

  #[test]
  fn lines_can_delete() {
    let mut l = Lines::from_slice(LINES.as_ref());

    l.update(1, 7, Vec::new());

    assert_eq!(l.0[0], OriginalLine(b"This"));
    assert_eq!(l.0[1], OriginalLine(b"."));
    assert_eq!(l.len(), 2);
  }

  #[test]
  fn lines_can_insert() {
    let mut l = Lines::from_slice(LINES.as_ref());
    let newlines = vec![
      "haaargl".to_string(),
      "waaarglll".to_string(),
      "blaaargl".to_string(),
    ];

    l.update(2, 2, newlines);

    assert_eq!(l.0[1], OriginalLine(b"is "));
    assert_eq!(l.0[2], ChangedLine("haaargl".to_string()));
    assert_eq!(l.0[5], OriginalLine(b"an "));
    assert_eq!(l.len(), 11);
  }

  #[test]
  fn lines_can_update() {
    let mut l = Lines::from_slice(LINES.as_ref());
    let newlines = vec![
      "haaargl".to_string(),
      "waaarglll".to_string(),
      "blaaargl".to_string(),
    ];

    l.update(1, 7, newlines);

    assert_eq!(l.0[0], OriginalLine(b"This"));
    assert_eq!(l.0[3], ChangedLine("blaaargl".to_string()));
    assert_eq!(l.0[4], OriginalLine(b"."));
    assert_eq!(l.len(), 5);
  }

  #[test]
  fn lines_from_file() {
    let v = Lines::read_file(file!()).unwrap();
    let l = Lines::from_slice(&v);
    let f = OriginalLine(
      b"//! This module holds the datastructure for the Lines of the \
             buffer. For now,",
    );

    assert_eq!(f, l.0[0]);
  }

}
