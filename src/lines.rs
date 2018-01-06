//! This module holds the datastructure for the Lines of the buffer. For now,
//! it's simply a `Vec<String>` with an appropriate API.
//!
//! Future ideas, if performance isn't enough: Skip list, gap buffer (adapted to
//! lines instead of strings), rope (adapted to lines instead of strings)
use std::ops;

use card::keyword::Keyword;

/// The struct to hold the lines.
#[derive(Debug)]
pub struct Lines(pub Vec<String>);

impl Lines {
  // Create a new Lines struct from a `Vec<String>`.
  pub fn new(v: Vec<String>) -> Lines {
    Lines { 0: v }
  }

  // Returns the number of lines
  pub fn len(&self) -> usize {
    self.0.len()
  }

  /// Update Lines:
  ///   * `firstline` is zero-indexed (just as Lines itself)
  ///   * If `numreplaced` is zero, the lines were added before `firstline`
  pub fn update(&mut self, first: u64, num: u64, linedata: Vec<String>) {
    let range = first as usize..(first as usize + num as usize);
    let _v = self.0.splice(range, linedata);
  }
}

impl ops::Index<usize> for Lines {
  type Output = String;

  fn index(&self, idx: usize) -> &String {
    &self.0[idx]
  }
}

impl ops::Deref for Lines {
  type Target = [String];

  fn deref(&self) -> &[String] {
    &self.0
  }
}

/// A datastructure to iterate over lines
///
/// `start_idx` is the index of the first line in the main `Lines` struct which
/// we need to pass upon createn, as `enumerate` always starts at 0.
pub struct LinesIter<'a, I: 'a, T>
where
  I: Iterator<Item = (usize, T)>,
  T: AsRef<str>,
{
  it: &'a mut I,
}

impl<'a, I: 'a, T> LinesIter<'a, I, T>
where
  I: Iterator<Item = (usize, T)>,
  T: AsRef<str>,
{
  pub fn skip_comments(&'a mut self) -> Option<(usize, T)> {
    while let Some((idx, line)) = self.it.next() {
      let kw = Keyword::parse(&line);
      if kw != Some(Keyword::Comment) {
        return Some((idx, line));
      }
    }

    return None;
  }
}

#[cfg(test)]
mod tests {
  use lines::Lines;
  use lines::LinesIter;

  const LINES: [&'static str; 8] =
    ["This", "is", "an", "example", "of", "some", "lines", "."];

  #[test]
  fn lines_can_delete() {
    let v = LINES.iter().map(|s| s.to_string()).collect();
    let mut l = Lines::new(v);

    l.update(1, 6, Vec::new());
    assert_eq!(l[0], "This");
    assert_eq!(l[1], ".");
    assert_eq!(l.len(), 2);
  }

  #[test]
  fn lines_can_insert() {
    let v = LINES.iter().map(|s| s.to_string()).collect();
    let mut l = Lines::new(v);

    let newlines = vec![
      "haaargl".to_string(),
      "waaarglll".to_string(),
      "blaaargl".to_string(),
    ];

    l.update(2, 0, newlines);
    assert_eq!(l[2], "haaargl");
    assert_eq!(l.len(), 11);
  }

  #[test]
  fn lines_can_update() {
    let v = LINES.iter().map(|s| s.to_string()).collect();
    let mut l = Lines::new(v);

    let newlines = vec![
      "haaargl".to_string(),
      "waaarglll".to_string(),
      "blaaargl".to_string(),
    ];

    l.update(1, 6, newlines);
    assert_eq!(l[0], "This");
    assert_eq!(l[3], "blaaargl");
    assert_eq!(l[4], ".");
    assert_eq!(l.len(), 5);
  }

  const COMMENTS: [&'static str; 8] = [
    "#This",
    "#is",
    "#an",
    "#example",
    "of",
    "some",
    "lines",
    ".",
  ];

  #[test]
  fn linesiter_works_with_slice() {
    let mut itr = COMMENTS.iter().enumerate();
    {
      let mut li = LinesIter { it: &mut itr };
      let nextline = li.skip_comments();
      assert_eq!(nextline, Some((4, &"of")));
    }
    assert_eq!(itr.next(), Some((5, &"some")));
  }

  #[test]
  fn linesiter_works_with_vec() {
    let v: Vec<String> = vec!["abc".to_owned(), "abc".to_owned()];

    let mut itr = v.iter().enumerate();
    {
      let mut li = LinesIter { it: &mut itr };
      let nextline = li.skip_comments();
      assert_eq!(nextline, Some((0, &"abc".to_owned())));
    }
    assert_eq!(itr.next(), Some((1, &"abc".to_owned())));
  }
}
