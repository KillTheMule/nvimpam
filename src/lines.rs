//! This module holds the datastructure for the Lines of the buffer. For now,
//! it's simply a `Vec<String>` with an appropriate API. Also home to
//! `LinesIter` which is used to iterate over the lines to parse them into the
//! fold structure.
//!
//! Future ideas, if performance isn't enough: Skip list, gap buffer (adapted to
//! lines instead of strings), rope (adapted to lines instead of strings)
use std::ops;

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

  // Check if there are any lines
  pub fn is_empty(&self) -> bool {
    self.0.is_empty()
  }

  /// Update Lines:
  ///   * `first` and `last` are zero-indexed (just as Lines itself)
  ///   * If `numreplaced` is zero, the lines were added before line `first`
  pub fn update(&mut self, first: u64, last: u64, linedata: Vec<String>) {
    let range = first as usize..last as usize;
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

#[cfg(test)]
mod tests {
  use lines::Lines;

  const LINES: [&'static str; 8] = [
    "This", "is", "an", "example", "of", "some", "lines", ".",
  ];

  #[test]
  fn lines_can_delete() {
    let v = LINES.iter().map(|s| s.to_string()).collect();
    let mut l = Lines::new(v);

    l.update(1, 7, Vec::new());
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

    l.update(2, 2, newlines);
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

    l.update(1, 7, newlines);
    assert_eq!(l[0], "This");
    assert_eq!(l[3], "blaaargl");
    assert_eq!(l[4], ".");
    assert_eq!(l.len(), 5);
  }

}
