//! This module holds the datastructure for the Lines of the buffer. For now,
//! it's simply a `Vec<String>` with an appropriate API.

use std::ops;

#[derive(Debug)]
pub struct Lines(pub Vec<String>);

impl Lines {
  // Create a new Lines struct from a `Vec<String>`.
  pub fn new(v: Vec<String>) -> Lines {
    Lines { 0: v }
  }

  /// Insert a String at the given index. Elements after that index are
  /// shifted.
  pub fn insert(&mut self, index: usize, element: String) {
    self.0.insert(index, element)
  }

  /// Remove an element of a given index and return it. Elements after that
  /// index are shifted.
  pub fn remove(&mut self, index: usize) -> String {
    self.0.remove(index)
  }

  /// Insert multiple elements at a given index. Elements after that index are
  /// shifted.
  pub fn insert_mult<I>(&mut self, index: usize, insert: I)
  where
    I: IntoIterator<Item = String>,
  {
    let v = self.0.splice(index..index, insert).collect();
    self.0 = v;
  }

  /// Remove multiple elemts starting at a given index. Elements after the
  /// deleted ones are shifted
  pub fn remove_mult(&mut self, index: usize, number: usize) {
    let v = self.0.splice(index..index + number, Vec::new()).collect();
    self.0 = v;
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

  fn index(&self, i: usize) -> &String {
    &self.0[i]
  }
}

impl ops::Deref for Lines {
  type Target = [String];

  fn deref(&self) -> &[String] {
    &self.0
  }
}
