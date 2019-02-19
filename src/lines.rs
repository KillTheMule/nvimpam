//! This module holds the datastructure for the Lines of the buffer.
//!
//! Future ideas, if performance isn't enough: Skip list, gap buffer (adapted to
//! lines instead of strings), rope (adapted to lines instead of strings)
use std::{convert::AsRef, fmt, ops::Deref, slice};

use crate::{card::keyword::Keyword, linenr::LineNr, linesiter::LinesIter};

/// An enum representing a line of a file, either as a byte slice (which we
/// obtain from reading a file into a `Vec<u8>` and splitting on newlines) or an
/// owned `String` (which we get from neovim's buffer update API via a
/// [`LinesEvent`](::event::Event::LinesEvent)).
#[derive(Debug, PartialEq)]
pub enum RawLine<'a> {
  OriginalLine(&'a [u8]),
  ChangedLine(String),
}

/// A struct to hold the data of a [`Line`](::lines::Line) that has been
/// [`parse`](::card::keyword::Keyword::parse)d before.
#[derive(PartialEq, Debug)]
pub struct ParsedLine<'a> {
  pub number: LineNr,
  pub text: RawLine<'a>,
  pub keyword: Option<Keyword>,
}

/// A struct to hold a [`Line`](::lines::Line) of a file that has been
/// [`parse`](::card::keyword::Keyword::parse)d and starts with a
/// [`Keyword`](::card::keyword::Keyword).
#[derive(PartialEq, Debug)]
pub struct KeywordLine<'a> {
  pub number: LineNr,
  pub text: &'a [u8],
  pub keyword: Keyword,
}

/// The struct to hold the lines.
#[derive(Debug, Default, PartialEq)]
pub struct Lines<'a>(Vec<ParsedLine<'a>>);

impl<'a> AsRef<[u8]> for RawLine<'a> {
  fn as_ref(&self) -> &[u8] {
    use self::RawLine::*;
    match self {
      OriginalLine(l) => l,
      ChangedLine(s) => s.as_ref(),
    }
  }
}

impl<'a> ParsedLine<'a> {
  fn shift(&mut self, added: isize) {
    self.number += added;
  }

  /// Try to convert the [`ParsedLine`](::lines::ParsedLine) into a
  /// [`KeywordLine`](::lines::KeywordLine). This is of course possible if and
  /// only if the [`keyword`](::lines::ParsedLine::keyword) is `Some(kw)`.
  pub fn try_into_keywordline(&'a self) -> Option<KeywordLine<'a>> {
    if let Some(kw) = self.keyword {
      return Some(KeywordLine {
        number: self.number,
        text: self.text.as_ref(),
        keyword: kw,
      });
    } else {
      return None;
    }
  }
}

impl<'a> Lines<'a> {
  pub fn new() -> Self {
    Lines(vec![])
  }

  pub fn is_empty(&self) -> bool {
    self.0.is_empty()
  }

  pub fn clear(&mut self) {
    self.0.clear()
  }
  pub fn len(&self) -> usize {
    self.0.len()
  }

  /// Extend a [`Lines`](::lines::Lines) struct from a `Vec<String>`
  pub fn parse_vec(&mut self, v: Vec<String>) {
    self.0.extend(
      v.into_iter()
        .enumerate()
        .filter(|(_, s)| {
          let first = s.as_bytes().get(0);
          first != Some(&b'$') && first != Some(&b'#')
        })
        .map(|(i, s)| ParsedLine {
          number: i.into(),
          keyword: Keyword::parse(s.as_ref()),
          text: RawLine::ChangedLine(s),
        }),
    );
  }

  /// Extend a [`Lines`](::lines::Lines) struct from a slice of `&'str`s
  pub fn parse_strs<'c: 'a>(&mut self, v: &'c [&'a str]) {
    self.0.extend(
      v.iter()
        .enumerate()
        .filter(|(_, s)| {
          let first = s.as_bytes().get(0_usize);
          first != Some(&b'$') && first != Some(&b'#')
        })
        .map(|(i, l)| ParsedLine {
          number: i.into(),
          text: RawLine::OriginalLine(l.as_ref()),
          keyword: Keyword::parse(l.as_ref()),
        }),
    );
  }

  /// Extend a [`Lines`](::lines::Lines) struct from a byte slice by splitting
  /// on newlines.
  pub fn parse_slice<'c: 'a>(&mut self, v: &'c [u8]) {
    self.0.extend(
      v.split(|b| *b == b'\n')
        .enumerate()
        .filter(|(_, s)| {
          let first = s.get(0_usize);
          first != Some(&b'$') && first != Some(&b'#')
        })
        .map(|(i, l)| ParsedLine {
          number: i.into(),
          text: RawLine::OriginalLine(l),
          keyword: Keyword::parse(l),
        }),
    );

    // If the file contains a final newline, we need to remove the empty slice
    // at the end
    if self.0.last().map(|p| p.text.as_ref()) == Some(b"") {
      self.0.pop();
    }
  }

  /// Update Lines:
  ///   * `first` and `last` are zero-indexed (just as Lines itself)
  ///   * `last` is the first line that has _not_ been updated
  /// This are the exact conditions to use the range `first..last` together with
  /// `splice` on a `Vec`.
  pub fn update(&mut self, first: LineNr, last: LineNr, linedata: Vec<String>) {
    let added: isize = linedata.len() as isize - (last - first);

    let startidx = self.linenr_to_index(first);
    let endidx = self.linenr_to_index(last);

    let indexrange = startidx..endidx;

    for line in self.0[indexrange.end..].iter_mut() {
      line.shift(added);
    }

    let mut newlines = Lines::new();
    newlines.parse_vec(linedata);

    let _ = self.0.splice(
      indexrange,
      newlines.0.into_iter().map(|mut p| {
        p.number += first;
        p
      }),
    );
  }

  /// Return an Iterator over the lines of a file.
  pub fn iter<'b>(&'a self) -> LinesIter<'b, slice::Iter<'b, ParsedLine<'b>>>
  where
    'a: 'b,
  {
    LinesIter::new(self.0.iter())
  }

  fn linenr_to_index(&self, line: LineNr) -> usize {
    self
      .0
      .binary_search_by_key(&line, |l| l.number)
      .unwrap_or_else(|e| e)
  }

  // TODO(KillTheMule): Efficient? This is called a lot ...
  // TODO(KillTheMule): This should return an option... none if empty
  /// Find the index of the first line that starts with a non-comment keyword
  /// before the line with the given number. If the line with the given number
  /// itself starts with a non-comment keyword, its index is returned.
  pub fn first_before(&self, line: LineNr) -> (usize, LineNr) {
    let line_index = self.linenr_to_index(line);
    let first_lnr = self
      .get(0)
      .map(|l| l.number)
      .unwrap_or_else(|| 0_usize.into());
    self
      .get(..=line_index)
      .unwrap_or(&[])
      .iter()
      .enumerate()
      .rfind(|(_, l)| l.keyword.is_some())
      .map(|(i, l)| (i, l.number))
      .unwrap_or((0, first_lnr))
  }

  // TODO(KillTheMule): Efficient? This is called a lot ...
  // TODO(KillTheMule): This should return an option... none if empty
  /// Find the index of the next line that starts with a non-comment keyword
  /// after the line with the given number. If the line with the given number
  /// itself starts with a non-comment keyword, its index is returned.
  pub fn first_after(&self, line: LineNr) -> (usize, LineNr) {
    let to_skip = self.linenr_to_index(line);
    let len = self.len();
    if len > 0 {
      let last_lnr = self[len - 1].number;
      self
        .iter()
        .enumerate()
        .skip(to_skip)
        .find(|(_, l)| l.keyword.is_some())
        .map(|(i, l)| (i, l.number))
        .unwrap_or((len - 1, last_lnr))
    } else {
      (0_usize, 0_usize.into())
    }
  }
}

/*
impl<'a> From<(LineNr, (&'a Option<Keyword>, &'a [u8]))> for ParsedLine<'a> {
  fn from(
    (u, (k, t)): (LineNr, (&'a Option<Keyword>, &'a [u8])),
  ) -> ParsedLine<'a> {
    ParsedLine {
      number: u,
      text: t,
      keyword: k.as_ref(),
    }
  }
}

impl<'a> From<(&'a Option<Keyword>, &'a Line<'a>)> for ParsedLine<'a> {
  fn from((k, l): (&'a Option<Keyword>, &'a Line<'a>)) -> ParsedLine<'a> {
    ParsedLine {
      number: l.nr(),
      text: l.text(),
      keyword: k.as_ref(),
    }
  }
}

impl<'a> From<(LineNr, (&'a Option<Keyword>, &'a Line<'a>))>
  for ParsedLine<'a>
{
  fn from(
    (u, (k, l)): (LineNr, (&'a Option<Keyword>, &'a Line<'a>)),
  ) -> ParsedLine<'a> {
    ParsedLine {
      number: u,
      text: l.text(),
      keyword: k.as_ref(),
    }
  }
}
*/

impl<'a> Deref for Lines<'a> {
  type Target = [ParsedLine<'a>];

  fn deref(&self) -> &[ParsedLine<'a>] {
    &self.0
  }
}

impl<'a> fmt::Display for RawLine<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    use self::RawLine::*;
    match self {
      OriginalLine(l) => {
        write!(f, "OriginalLine {{ {} }}", String::from_utf8_lossy(l))
      }
      ChangedLine(s) => write!(f, "ChangedLine {{ {} }}", s),
    }
  }
}

impl<'a> fmt::Display for ParsedLine<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{{{}, {}, {:?}}}", self.number, self.text, self.keyword)
  }
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

impl<'a> fmt::Display for Lines<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut s = String::new();
    s.push_str("Lines {{\n");
    for line in &self.0 {
      s.push_str(&format!(" {}\n", line));
    }
    s.push_str("}}\n");
    write!(f, "{}", s)
  }
}

#[cfg(test)]
mod tests {
  use crate::{linenr::LineNr, lines::Lines};
  use std::fs;

  const LINES: &str = "This\nis \nan \nexample \nof \nsome \nlines \n.";

  const LINES_DEL: &str = "This\n.";

  #[test]
  fn lines_can_delete() {
    let mut l = Lines::new();
    l.parse_slice(LINES.as_ref());

    let mut ln = Lines::new();
    ln.parse_slice(LINES_DEL.as_ref());

    l.update(1.into(), 7.into(), Vec::new());

    assert_eq!(l, ln);
  }

  const LINES_INS: &str = "This\nis \nhaaargl\nwaaarglll\nblaaargl\nan \
                           \nexample \nof \nsome \nlines \n.";
  #[test]
  fn lines_can_insert() {
    let mut l = Lines::new();
    l.parse_slice(LINES.as_ref());

    let mut ln = Lines::new();
    ln.parse_slice(LINES_INS.as_ref());

    let newlines = vec![
      "haaargl".to_string(),
      "waaarglll".to_string(),
      "blaaargl".to_string(),
    ];

    l.update(2.into(), 2.into(), newlines);

    for i in 0..11 {
      assert_eq!(
        (l[i].number, l[i].text.as_ref(), l[i].keyword),
        (ln[i].number, ln[i].text.as_ref(), ln[i].keyword)
      );
    }
  }

  const LINES_UPD: &str = "This\nhaaargl\nwaaarglll\nblaaargl\n.";

  #[test]
  fn lines_can_update() {
    let mut l = Lines::new();
    l.parse_slice(LINES.as_ref());

    let mut ln = Lines::new();
    ln.parse_slice(LINES_UPD.as_ref());
    let newlines = vec![
      "haaargl".to_string(),
      "waaarglll".to_string(),
      "blaaargl".to_string(),
    ];

    l.update(1.into(), 7.into(), newlines);

    for i in 0..5 {
      assert_eq!(
        (l[i].number, l[i].text.as_ref(), l[i].keyword),
        (ln[i].number, ln[i].text.as_ref(), ln[i].keyword)
      );
    }
  }

  #[test]
  fn lines_from_file() {
    let v = fs::read(file!()).unwrap();
    let mut l = Lines::new();
    l.parse_slice(&v);
    let f = "OriginalLine { //! This module holds the datastructure for the \
             Lines of the buffer. }"
      .to_string();

    assert_eq!(f, format!("{}", l.0[0].text));
  }

  macro_rules! test_before {
    ($lines: expr, $a: expr, $b: expr) => {
      assert_eq!(LineNr::from_usize($a), $lines.first_before($b.into()).1);
    };
  }
  macro_rules! test_after {
    ($lines: expr, $a: expr, $b: expr) => {
      assert_eq!(LineNr::from_usize($a), $lines.first_after($b.into()).1);
    };
  }

  const LINES2: &str = "x\nx\nNODE  / \nx\nx\nNODE  / \nx";

  #[test]
  fn first_before_after() {
    let mut lines = Lines::new();
    lines.parse_slice(LINES2.as_ref());

    test_before!(lines, 2, 2);
    test_after!(lines, 2, 2);
    test_before!(lines, 2, 4);
    test_after!(lines, 5, 4);
    test_before!(lines, 0, 1);
    test_after!(lines, 6, 7);
  }

  const LINES3: &str = "NODE  / 1";
  const LINES4: &str = "# kommentar";

  #[test]
  fn first_oneline() {
    let mut lines = Lines::new();
    lines.parse_slice(LINES3.as_ref());
    test_before!(lines, 0, 0);
    test_after!(lines, 0, 0);

    lines.clear();
    lines.parse_slice(LINES4.as_ref());
    test_before!(lines, 0, 0);
    test_after!(lines, 0, 0);
    test_before!(lines, 0, 1);
    test_after!(lines, 0, 1);
  }

}
