//! This module holds the datastructure for the Lines of the buffer. For now,
//! it's simply a `Vec<String>` with an appropriate API. Also home to
//! `LinesIter` which is used to iterate over the lines to parse them into the
//! fold structure.
//!
//! Future ideas, if performance isn't enough: Skip list, gap buffer (adapted to
//! lines instead of strings), rope (adapted to lines instead of strings)
use std::ops;

use card::keyword::Keyword;
use card::ges::GesType;

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
pub struct LinesIter<'a, I, T: 'a>
where
  I: Iterator<Item = (usize, &'a T)>,
  T: AsRef<str>,
{
  it: I,
}

impl<'a, I, T: 'a> LinesIter<'a, I, T>
where
  I: Iterator<Item = (usize, &'a T)>,
  T: AsRef<str>,
{
  /// Advance the iterator until meeting the first line that is not a
  /// comment. Return the index and a reference to that line. If all lines
  /// are comments, return `None`.
  pub fn skip_comments<'b>(&'b mut self) -> Option<(usize, &'a T)> {
    self.it.find(|&(_, ref l)| {
      Keyword::parse(l) != Some(Keyword::Comment)
    })
  }

  /// Advance the iterator until meeting the first line with a keyword. Return
  /// the index and a reference to that line. If no line starts with a
  /// keyword, return `None`.
  ///
  /// NOTE: A Comment line counts as a keyword. Also see
  /// `skip_to_next_real_keyword`.
  pub fn skip_to_next_keyword<'b>(&'b mut self) -> Option<(usize, &'a T)> {
    self.it.find(|&(_, ref l)| Keyword::parse(l).is_some())
  }

  /// Advance the iterator until meeting the first line with a keyword that's
  /// not a comment. Return the index and a reference to that line. If no
  /// line starts with a keyword, return `None`.
  pub fn skip_to_next_real_keyword<'b>(&'b mut self) -> Option<(usize, &'a T)> {
    self.it.find(|&(_, ref l)| {
      let kw = Keyword::parse(l);
      kw.is_some() && kw != Some(Keyword::Comment)
    })
  }

  /// Advance the iterator until the first line after a General Entity
  /// Selection
  /// (GES). Return the index and a reference to that line. If the GES
  /// includes
  /// the last line of the file, return `None`.
  pub fn skip_ges<'b>(&'b mut self, ges: &GesType) -> Option<(usize, &'a T)> {
    let mut idx;
    let mut line;

    let tmp = self.it.next();
    match tmp {
      None => return None,
      Some((i, l)) => {
        idx = i;
        line = l;
      }
    }

    while ges.contains(&line) {
      let tmp = self.it.next();
      match tmp {
        None => return None,
        Some((i, l)) => {
          idx = i;
          line = l;
        }
      }
    }

    if ges.ended_by(&line) {
      // Here: Last line ends the ges, just need to fetch the next and return
      // the data
      let tmp = self.it.next();
      match tmp {
        None => return None,
        t @ Some(_) => return t,
      }
    } else {
      // Ges implicitely ended, so it does not encompass the current line
      return Some((idx, line));
    }
  }
}

#[cfg(test)]
mod tests {
  use lines::Lines;
  use lines::LinesIter;
  use card::ges::GesType;

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

  const KEYWORD_LINES: [&'static str; 8] = [
    "#Comment",
    "   nokeyword",
    "NODE  / ",
    "#example",
    "NSMAS / ",
    "some",
    "lines",
    ".",
  ];

  #[test]
  fn linesiter_needs_no_keywords() {
    let mut itr = COMMENTS.iter().skip(4).enumerate();
    let mut li = LinesIter { it: &mut itr };
    assert_eq!(li.skip_to_next_keyword(), None);
  }

  #[test]
  fn linesiter_finds_keywords() {
    let mut itr = KEYWORD_LINES.iter().enumerate();
    {
      let mut li = LinesIter { it: &mut itr };
      assert_eq!(li.skip_to_next_keyword(), Some((0, &"#Comment")));
      assert_eq!(li.skip_to_next_keyword(), Some((2, &"NODE  / ")));
      assert_eq!(li.skip_to_next_keyword(), Some((3, &"#example")));
      assert_eq!(li.skip_to_next_keyword(), Some((4, &"NSMAS / ")));
      assert_eq!(li.skip_to_next_keyword(), None);
    }
    assert_eq!(itr.next(), None);
  }

  #[test]
  fn linesiter_finds_real_keywords() {
    let mut itr = KEYWORD_LINES.iter().enumerate();
    {
      let mut li = LinesIter { it: &mut itr };
      assert_eq!(li.skip_to_next_real_keyword(), Some((2, &"NODE  / ")));
      assert_eq!(li.skip_to_next_real_keyword(), Some((4, &"NSMAS / ")));
      assert_eq!(li.skip_to_next_real_keyword(), None);
    }
    assert_eq!(itr.next(), None);
  }

  const GES1: [&'static str; 5] = [
    "        PART 1234",
    "        OGRP 'hausbau'",
    "        DELGRP>NOD 'nix'",
    "        END",
    "NODE  / ",
  ];

  #[test]
  fn ges_can_be_skipped() {
    let g = GesType::GesNode;
    let mut itr = GES1.iter().enumerate();
    let mut li = LinesIter { it: &mut itr };

    assert_eq!(li.skip_ges(&g), Some((4, &"NODE  / ")));
  }

  const GES2: [&'static str; 9] = [
    "        PART 1234",
    "        OGRP 'hausbau'",
    "        END",
    "        DELGRP>NOD 'nix'",
    "        MOD 10234",
    "        NOD 1 23 093402 82",
    "        END_MOD",
    "        DELELE 12",
    "        END",
  ];

  #[test]
  fn ges_can_be_skipped_repeatedly() {
    let g = GesType::GesNode;
    let mut itr = GES2.iter().enumerate();
    let mut li = LinesIter { it: &mut itr };

    assert_eq!(li.skip_ges(&g), Some((3, &"        DELGRP>NOD 'nix'")));
    assert_eq!(li.skip_ges(&g), None);
  }

  const GES3: [&'static str; 9] = [
    "        PART 1234",
    "        OGRP 'hausbau'",
    "NODE  /         END",
    "        DELGRP>NOD 'nix'",
    "        MOD 10234",
    "        NOD 1 23 093402 82",
    "        END_MOD",
    "Whatever",
    "        END",
  ];

  #[test]
  fn ges_ends_without_end() {
    let g = GesType::GesNode;
    let mut itr = GES3.iter().enumerate();
    let mut li = LinesIter { it: &mut itr };

    assert_eq!(li.skip_ges(&g), Some((2, &"NODE  /         END")));
    assert_eq!(li.skip_ges(&g), Some((7, &"Whatever")));
  }

  const GES4: [&'static str; 2] = ["wupdiwup", "NODE  / "];

  #[test]
  fn ges_can_skip_nothing() {
    let g = GesType::GesNode;
    let mut itr = GES4.iter().enumerate();
    let mut li = LinesIter { it: &mut itr };

    assert_eq!(li.skip_ges(&g), Some((0, &"wupdiwup")));
  }

  const GES6: [&'static str; 7] = [
    "        PART 1234",
    "#Comment here",
    "        OGRP 'hausbau'",
    "        DELGRP>NOD 'nix'",
    "        END",
    "$Another comment",
    "NODE  / ",
  ];

  #[test]
  fn ges_includes_comments_inside() {
    let g = GesType::GesNode;
    let mut itr = GES6.iter().enumerate();
    let mut li = LinesIter { it: &mut itr };

    assert_eq!(li.skip_ges(&g), Some((5, &"$Another comment")));
  }
}
