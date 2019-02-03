//! This module holds the datastructure for the Lines of the buffer.
//!
//! Future ideas, if performance isn't enough: Skip list, gap buffer (adapted to
//! lines instead of strings), rope (adapted to lines instead of strings)
use std::{
  convert::{AsRef, From},
  fmt,
  fs::File,
  io::Read,
  ops::Deref,
  path::Path,
};

use failure::{Error, ResultExt};

use crate::card::keyword::Keyword;

/// An enum representing a line of a file, either as a byte slice (which we
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

impl<'a> fmt::Display for Line<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    use self::Line::*;
    match self {
      OriginalLine(l) => write!(f, "Line {{ {} }}", String::from_utf8_lossy(l)),
      ChangedLine(s) => write!(f, "Line {{ {} }}", s),
    }
  }
}

/// The struct to hold the lines.
#[derive(Debug, Default)]
pub struct Lines<'a>(Vec<Line<'a>>);

impl<'a> fmt::Display for Lines<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut s = String::new();
    s.push_str("Lines {{\n");
    for line in self.iter() {
      s.push_str(&format!(" {{{}}}\n", String::from_utf8_lossy(line)));
    }
    s.push_str("}}\n");
    write!(f, "{}", s)
  }
}

/// A struct returned by the [`iter()`](::lines::Lines::iter) method of
/// [`Lines`](::lines::Lines). Used to iterate over the [`Line`](::lines::Line)
/// of a file.
pub struct LinesIter<'a, I>
where
  I: Iterator<Item = &'a Line<'a>>,
{
  li: I,
}

impl<'a> Lines<'a> {
  pub fn new() -> Self {
    Lines(vec![])
  }

  pub fn clear(&mut self) {
    self.0.clear()
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
  pub fn from_vec(&mut self, v: Vec<String>) {
    self.0.extend(v.into_iter().map(Line::ChangedLine))
  }

  /// Creates a new `Lines` struct from a slice of `&'str`s
  pub fn from_strs<'c: 'a>(&mut self, v: &'c [&'a str]) {
    self
      .0
      .extend(v.iter().map(|l| Line::OriginalLine(l.as_ref())));
  }

  /// Create a new `Lines` struct from a byte slice by splitting on newlines.
  pub fn from_slice<'c: 'a>(&mut self, v: &'c [u8]) {
    self
      .0
      .extend(v.split(|b| *b == b'\n').map(Line::OriginalLine));

    // If the file contains a final newline, we need to remove the empty slice
    // at the end
    if self.0.last() == Some(&Line::OriginalLine(b"")) {
      self.0.pop();
    }
  }

  /// Read a file into a `Vec<u8>`. For usage with
  /// [`from_slice`](::lines::Lines::from_slice).
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
  /// This are the exact conditions to use the range `first..last` together with
  /// `splice` on a `Vec`.
  pub fn update(&mut self, first: u64, last: u64, linedata: Vec<String>) {
    let range = first as usize..last as usize;
    let _ = self
      .0
      .splice(range, linedata.into_iter().map(Line::ChangedLine));
  }

  /// Return an Iterator over the lines of a file.
  pub fn iter(&'a self) -> LinesIter<'a, impl Iterator<Item = &'a Line<'a>>> {
    LinesIter { li: self.0.iter() }
  }
}

impl<'a, I> Iterator for LinesIter<'a, I>
where
  I: Iterator<Item = &'a Line<'a>>,
{
  type Item = &'a [u8];

  fn next(&mut self) -> Option<Self::Item> {
    self.li.next().map(|o| o.as_ref())
  }
}

impl<'a> From<Vec<String>> for Lines<'a> {
  fn from(v: Vec<String>) -> Lines<'a> {
    Lines(v.into_iter().map(Line::ChangedLine).collect())
  }
}

/// A struct to hold the data of a [`Line`](::lines::Line) that has been
/// [`parse`](::card::keyword::Keyword::parse)d before.
#[derive(PartialEq, Debug)]
pub struct ParsedLine<'a> {
  pub number: usize,
  pub text: &'a [u8],
  pub keyword: Option<&'a Keyword>,
}

impl<'a> ParsedLine<'a> {
  /// Try to convert the [`ParsedLine`](::lines::ParsedLine) into a
  /// [`KeywordLine`](::lines::KeywordLine). This is of course possible if and
  /// only if the [`keyword`](::lines::ParsedLine::keyword) is `Some(kw)`.
  pub fn try_into_keywordline(&self) -> Option<KeywordLine<'a>> {
    if let Some(kw) = self.keyword {
      return Some(KeywordLine {
        number: self.number,
        text: self.text,
        keyword: kw,
      });
    } else {
      return None;
    }
  }
}

impl<'a> From<(usize, (&'a Option<Keyword>, &'a [u8]))> for ParsedLine<'a> {
  fn from(
    (u, (k, t)): (usize, (&'a Option<Keyword>, &'a [u8])),
  ) -> ParsedLine<'a> {
    ParsedLine {
      number: u,
      text: t,
      keyword: k.as_ref(),
    }
  }
}

impl<'a> From<(usize, (&'a Option<Keyword>, &'a Line<'a>))> for ParsedLine<'a> {
  fn from(
    (u, (k, t)): (usize, (&'a Option<Keyword>, &'a Line<'a>)),
  ) -> ParsedLine<'a> {
    ParsedLine {
      number: u,
      text: t.as_ref(),
      keyword: k.as_ref(),
    }
  }
}

impl<'a> From<KeywordLine<'a>> for ParsedLine<'a> {
  fn from(kl: KeywordLine<'a>) -> ParsedLine<'a> {
    ParsedLine {
      number: kl.number,
      text: kl.text,
      keyword: Some(kl.keyword),
    }
  }
}

impl<'a> fmt::Display for ParsedLine<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "ParsedLine {{{}, text: {}, keyword: {:?}}}",
      self.number,
      String::from_utf8_lossy(self.text),
      self.keyword
    )
  }
}

/// A struct to hold a [`Line`](::lines::Line) of a file that has been
/// [`parse`](::card::keyword::Keyword::parse)d and starts with a
/// [`Keyword`](::card::keyword::Keyword).
#[derive(PartialEq, Debug)]
pub struct KeywordLine<'a> {
  pub number: usize,
  pub text: &'a [u8],
  pub keyword: &'a Keyword,
}

impl<'a> fmt::Display for KeywordLine<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "KeywordLine {{{}, text: {}, keyword: {:?}}}",
      self.number,
      String::from_utf8_lossy(self.text),
      self.keyword
    )
  }
}

impl<'a> Deref for Lines<'a> {
  type Target = [Line<'a>];

  fn deref(&self) -> &[Line<'a>] {
    &self.0
  }
}

#[cfg(test)]
mod tests {
  use crate::lines::{Line::*, Lines};

  const LINES: &str = "This\nis \nan \nexample \nof \nsome \nlines \n.";

  #[test]
  fn lines_can_delete() {
    let mut l = Lines::new();
    l.from_slice(LINES.as_ref());

    l.update(1, 7, Vec::new());

    assert_eq!(l.0[0], OriginalLine(b"This"));
    assert_eq!(l.0[1], OriginalLine(b"."));
    assert_eq!(l.len(), 2);
  }

  #[test]
  fn lines_can_insert() {
    let mut l = Lines::new();
    l.from_slice(LINES.as_ref());
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
    let mut l = Lines::new();
    l.from_slice(LINES.as_ref());
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
    let mut l = Lines::new();
    l.from_slice(&v);
    let f = OriginalLine(
      b"//! This module holds the datastructure for the Lines of the \
             buffer.",
    );

    assert_eq!(f, l.0[0]);
  }

}
