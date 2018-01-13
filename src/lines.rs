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
  /// Selection (GES). Return the index, a reference to that line and the
  /// index of the first line after the GES.
  ///
  /// Corner cases:
  ///  * If the GES is ended by the END keyword
  ///    - Return the next line in the first Option, and its index
  ///      in the second (redundantly). If there's no next line (EOF), return
  ///      `(None, None)`.
  ///  * If the GES is ended implicitely
  ///    - If there are no comment lines after it, return the following line
  ///      in the first Option, and its index in the second (redundantly). If
  ///      the file ends after the GES, return `(None, None)`.
  ///    - If there are comment lines after it, return the first non-comment
  ///      line in the first Option (if the file ends before that, return
  ///      `None`), and the index of the first comment line after the GES
  ///      in the second option.
  ///
  pub fn skip_ges<'b>(
    &'b mut self,
    ges: &GesType,
  ) -> (Option<(usize, &'a T)>, Option<usize>) {
    let mut idx;
    let mut line;
    let mut line_is_comment = false;
    let mut ges_contains_line;
    let mut first_comment_idx = None;

    let tmp = self.it.next();
    match tmp {
      None => return (None, None),
      Some((i, l)) => {
        idx = i;
        line = l;
        ges_contains_line = ges.contains(&line);
        if !ges_contains_line {
          line_is_comment = Keyword::parse(&line) == Some(Keyword::Comment);
          if line_is_comment {
            first_comment_idx = Some(idx);
          }
        }
      }
    }

    while ges_contains_line || line_is_comment {
      let tmp = self.it.next();
      match tmp {
        None => {
          if let Some(i) = first_comment_idx {
            return (None, Some(i));
          } else {
            return (None, Some(idx));
          }
        }
        Some((i, l)) => {
          ges_contains_line = ges.contains(&l);
          if !ges_contains_line {
            line_is_comment = Keyword::parse(&l) == Some(Keyword::Comment);
            if line_is_comment && first_comment_idx.is_none() {
              first_comment_idx = Some(i);
            }
          }
          // Keep order! If `ges_contains_line`, the other variable might just
          // be wrong, we only set it when really necessary.
          if ges_contains_line || !line_is_comment {
            first_comment_idx = None;
          }
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
        None => {
            return (None, None);
        }
        Some((i, l)) => return (Some((i, l)), Some(i)),
      }
    } else {
      // Ges implicitely ended, so it does not encompass the current line
      if let Some(i) = first_comment_idx {
        return (Some((idx, line)), Some(i));
      } else {
        return (Some((idx, line)), Some(idx));
      }
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
      assert_eq!(nextline, Some((4, &COMMENTS[4])));
    }
    assert_eq!(itr.next(), Some((5, &COMMENTS[5])));
  }

  #[test]
  fn linesiter_works_with_vec() {
    let v: Vec<String> = vec!["abc".to_owned(), "abc".to_owned()];

    let mut itr = v.iter().enumerate();
    {
      let mut li = LinesIter { it: &mut itr };
      let nextline = li.skip_comments();
      assert_eq!(nextline, Some((0, &v[0])));
    }
    assert_eq!(itr.next(), Some((1, &v[1])));
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
      assert_eq!(li.skip_to_next_keyword(), Some((0, &KEYWORD_LINES[0])));
      assert_eq!(li.skip_to_next_keyword(), Some((2, &KEYWORD_LINES[2])));
      assert_eq!(li.skip_to_next_keyword(), Some((3, &KEYWORD_LINES[3])));
      assert_eq!(li.skip_to_next_keyword(), Some((4, &KEYWORD_LINES[4])));
      assert_eq!(li.skip_to_next_keyword(), None);
    }
    assert_eq!(itr.next(), None);
  }

  #[test]
  fn linesiter_finds_real_keywords() {
    let mut itr = KEYWORD_LINES.iter().enumerate();
    {
      let mut li = LinesIter { it: &mut itr };
      assert_eq!(li.skip_to_next_real_keyword(), Some((2, &KEYWORD_LINES[2])));
      assert_eq!(li.skip_to_next_real_keyword(), Some((4, &KEYWORD_LINES[4])));
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

    assert_eq!(li.skip_ges(&g), (Some((4, &GES1[4])), Some(4)));
    assert_eq!(li.it.next(), None);
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

    assert_eq!(li.skip_ges(&g), (Some((3, &GES2[3])), Some(3)));
    assert_eq!(li.skip_ges(&g), (None, None));
    assert_eq!(li.it.next(), None);
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

    assert_eq!(li.skip_ges(&g), (Some((2, &GES3[2])), Some(2)));
    assert_eq!(li.skip_ges(&g), (Some((7, &GES3[7])), Some(7)));
    assert_eq!(li.it.next(), Some((8, &GES3[8])));
  }

  const GES4: [&'static str; 2] = ["wupdiwup", "NODE  / "];

  #[test]
  fn ges_can_skip_nothing() {
    let g = GesType::GesNode;
    let mut itr = GES4.iter().enumerate();
    let mut li = LinesIter { it: &mut itr };

    assert_eq!(li.skip_ges(&g), (Some((0, &GES4[0])), Some(0)));
    assert_eq!(li.it.next(), Some((1, &GES4[1])));
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

    assert_eq!(li.skip_ges(&g), (Some((5, &GES6[5])), Some(5)));
    assert_eq!(li.it.next(), Some((6, &GES6[6])));
  }

  const GES7: [&'static str; 4] = [
    "#        PART 1234",
    "#Comment here",
    "$Another comment",
    "#NODE  / ",
  ];

  #[test]
  fn ges_works_with_only_comments() {
    let g = GesType::GesNode;
    let mut itr = GES7.iter().enumerate();
    let mut li = LinesIter { it: &mut itr };

    assert_eq!(li.skip_ges(&g), (None, Some(0)));
    assert_eq!(li.it.next(), None);
  }

  const GES8: [&'static str; 4] = [
    "        PART 1234",
    "#Comment here",
    "$Another comment",
    "#NODE  / ",
  ];

  #[test]
  fn ges_skips_over_comments_after_end() {
    let g = GesType::GesNode;
    let mut itr = GES8.iter().enumerate();
    let mut li = LinesIter { it: &mut itr };

    assert_eq!(li.skip_ges(&g), (None, Some(1)));
    assert_eq!(li.it.next(), None);
  }
}
