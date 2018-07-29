//! This module holds [`NoCommentIter`](NoCommentIter), the central
//! datastructure for the folding functionality of nvimpam.
//!
//! It returns enumerated Lines, but skips Comments (lines starting with `$` or
//! `#`). All skip functions, used by
//! [`add_folds`](::folds::FoldList::add_folds), work on a
//! [`NoCommentIter`](NoCommentIter).
use std::default::Default;

use card::ges::GesType;
use card::keyword::Keyword;
use card::line::{CondResult, Line};
use card::Card;
use skipresult::{SkipLine, SkipResult};

// Used in skip_ges to get the next line. If it's None, we're at the end of
// the file and only return what we found before.
macro_rules! next_or_return_previdx {
  ($self:ident, $prevline:ident) => {
    match $self.next() {
      None => {
        return SkipResult {
          skip_end: $prevline.map(|o: (usize, &'a T)| o.0),
          ..Default::default()
        }
      }
      Some(t) => t,
    };
  };
}

macro_rules! next_or_return_none {
  ($self:ident) => {
    match $self.next() {
      None => return None,
      Some(t) => t,
    };
  };
}

/// Designates that the comments have been removed.
pub trait CommentLess {
  fn remove_comments(self) -> NoCommentIter<Self>
  where
    Self: Sized;
}

/// The struct simply holds a type instance. Skipping comments is done in the
/// Iterator implementation.
pub struct NoCommentIter<I> {
  it: I,
}

impl<'a, I, T: 'a> Iterator for NoCommentIter<I>
where
  T: AsRef<str>,
  I: Iterator<Item = (usize, &'a T)>,
{
  type Item = (usize, &'a T);

  fn next(&mut self) -> Option<Self::Item> {
    while let Some((i, n)) = self.it.next() {
      let t = n.as_ref().as_bytes();
      if !(t.len() > 0 && (t[0] == b'#' || t[0] == b'$')) {
        return Some((i, n));
      }
    }
    None
  }
}

impl<I> CommentLess for I {
  fn remove_comments(self) -> NoCommentIter<I> {
    NoCommentIter { it: self }
  }
}

impl<'a, I, T: 'a> NoCommentIter<I>
where
  I: Iterator<Item = (usize, &'a T)>,
  T: AsRef<str>,
{
  /// Advance the iterator until meeting the first line with a keyword. If the
  /// file ends before that, return the default
  /// [SkipResult](::skipresult::SkipResult), with
  /// [skipend](::skipresult::SkipResult.skipend) set to the index of the last
  /// line of the file.
  pub fn skip_to_next_keyword<'b>(&'b mut self) -> Option<SkipLine<'a, T>> {
    let mut line: (usize, &'a T) = next_or_return_none!(self);

    loop {
      match Keyword::parse(line.1) {
        None => line = next_or_return_none!(self),
        Some(line_kw) => return Some(SkipLine { line, line_kw }),
      }
    }
  }

  /// Advance the iterator until the first line after a General Entity
  /// Selection (GES).
  ///
  pub fn skip_ges<'b>(&'b mut self, ges: GesType) -> SkipResult<'a, T> {
    let mut prevline: Option<(usize, &'a T)> = None;
    let mut nextline: (usize, &'a T) = next_or_return_previdx!(self, prevline);

    while ges.contains(nextline.1) {
      prevline = Some(nextline);
      nextline = next_or_return_previdx!(self, prevline);
    }

    if ges.ended_by(&nextline.1) {
      prevline = Some(nextline);
      nextline = next_or_return_previdx!(self, prevline);
    }

    SkipResult {
      nextline: Some(nextline),
      nextline_kw: Keyword::parse(&nextline.1),
      skip_end: prevline.map(|o| o.0),
    }
  }

  /// A wrapper around [`skip_card`](NoCommentIter::skip_card) and
  /// [`skip_card_gather`](NoCommentIter::skip_card_gather), dispatching by
  /// value of [`Card.ownfold`](::card::Card)
  pub fn skip_fold<'b>(
    &'b mut self,
    skipline: &SkipLine<'a, T>,
  ) -> SkipResult<'a, T> {
    let card: &Card = (&skipline.line_kw).into();

    if card.ownfold {
      self.skip_card(skipline)
    } else {
      self.skip_card_gather(skipline)
    }
  }

  /// Let [`NoCommentIter`](NoCommentIter) skip the given
  /// [`Card`](::card::Card), but only skip this 1 card. This only really makes
  /// sense when the last line the iterator returned is the line with the
  /// keyword starting that card, which is passed as `skipline`.
  ///
  /// If you want to skip all cards of a given type, use
  /// [`skip_card_gather`](NoCommentIter::skip_card_gather)
  pub fn skip_card<'b>(
    &'b mut self,
    skipline: &SkipLine<'a, T>,
  ) -> SkipResult<'a, T> {
    let mut conds: Vec<CondResult> = vec![]; // the vec to hold the conditionals
    let mut cardlines = <&Card>::from(&skipline.line_kw).lines.iter();
    let cardline = cardlines.next().unwrap_or_else(|| unreachable!());

    if let Line::Provides(_s, ref c) = cardline {
      conds.push(c.evaluate(skipline.line.1));
    }

    let mut prevline: Option<(usize, &'a T)> = None;
    let mut nextline: (usize, &'a T) = next_or_return_previdx!(self, prevline);
    let mut nextline_kw = Keyword::parse(nextline.1);

    for cardline in cardlines {
      if nextline_kw.is_some() {
        return SkipResult {
          nextline: Some(nextline),
          nextline_kw,
          skip_end: prevline.map(|o| o.0),
        };
      }

      match *cardline {
        Line::Provides(_s, ref c) => {
          conds.push(c.evaluate(&nextline.1));
          prevline = Some(nextline);
          nextline = next_or_return_previdx!(self, prevline);
          nextline_kw = Keyword::parse(nextline.1);
        }
        Line::Ges(ref g) => {
          let contains = g.contains(nextline.1);
          let ended = g.ended_by(nextline.1);
          if contains || ended {
            if ended {
              prevline = Some(nextline);
              nextline = next_or_return_previdx!(self, prevline);
              nextline_kw = Keyword::parse(nextline.1);
            } else {
              let tmp = self.skip_ges(*g);

              match tmp.nextline {
                None => {
                  return SkipResult {
                    skip_end: tmp.skip_end,
                    ..Default::default()
                  }
                }
                Some((i, l)) => {
                  prevline = Some(nextline);
                  nextline = (i, l);
                  nextline_kw = Keyword::parse(nextline.1);
                }
              }
            }
          }
        }
        Line::Cells(_s) => {
          prevline = Some(nextline);
          nextline = next_or_return_previdx!(self, prevline);
          nextline_kw = Keyword::parse(nextline.1);
        }
        Line::Optional(_s, i) => {
          if conds.get(i as usize) == Some(&CondResult::Bool(true)) {
            prevline = Some(nextline);
            nextline = next_or_return_previdx!(self, prevline);
            nextline_kw = Keyword::parse(nextline.1);
          } else {
            continue;
          }
        }
        Line::Repeat(_s, i) => {
          let num = match conds.get(i as usize) {
            Some(CondResult::Number(Some(u))) if *u > 0 => u,
            _ => continue,
          };

          for _ in 0..*num-1 {
            let _ = self.next();
          }

          prevline = Some(nextline);
          nextline = next_or_return_previdx!(self, prevline);
          nextline_kw = Keyword::parse(nextline.1);
        }
        Line::Block(_l, s) => loop {
          if nextline_kw.is_some() {
            break;
          } else if nextline.1.as_ref().starts_with(s) {
            prevline = Some(nextline);
            nextline = next_or_return_previdx!(self, prevline);
            nextline_kw = Keyword::parse(nextline.1);
            break;
          } else {
            prevline = Some(nextline);
            nextline = next_or_return_previdx!(self, prevline);
            nextline_kw = Keyword::parse(nextline.1);
          }
        }
        Line::OptionalBlock(s1, s2) => {
          if !nextline.1.as_ref().starts_with(s1) {
            continue
          }
          loop {
            if nextline_kw.is_some() {
              break;
            } else if nextline.1.as_ref().starts_with(s2) {
              prevline = Some(nextline);
              nextline = next_or_return_previdx!(self, prevline);
              nextline_kw = Keyword::parse(nextline.1);
              break;
            } else {
              prevline = Some(nextline);
              nextline = next_or_return_previdx!(self, prevline);
              nextline_kw = Keyword::parse(nextline.1);
            }
          }
        }
      }
    }
    SkipResult {
      nextline: Some(nextline),
      nextline_kw,
      skip_end: prevline.map(|o| o.0),
    }
  }

  /// Let [`NoCommentIter`](NoCommentIter) skip all given
  /// [`Card`](::card::Card)s, until the next card starts. The basic assumption
  /// is that the last line the iterator returned is a the first line of a card
  /// of the given type, but that might not always be strictly neccessary.
  pub fn skip_card_gather<'b>(
    &'b mut self,
    nextline: &SkipLine<'a, T>,
  ) -> SkipResult<'a, T> {
    let mut curkw;
    let mut res;
    let mut curidx;
    let mut curline;
    let mut previdx = None;

    let card: &Card = (&nextline.line_kw).into();

    res = self.skip_card(nextline);

    loop {
      match res.nextline {
        // file ended before the next non-comment line
        None => {
          return SkipResult {
            skip_end: res.skip_end,
            ..Default::default()
          }
        }
        Some((i, l)) => {
          curkw = Keyword::parse(l);
          curline = l;
          curidx = i;
          // TODO: check this -1
          previdx = previdx.or_else(|| Some(curidx - 1));
        }
      }
      if curkw != Some(card.keyword) {
        break;
      } else {
        previdx = Some(curidx);
      }

      res = self.skip_card(&SkipLine {
        line: (curidx, curline),
        line_kw: curkw.unwrap(),
      });
    }

    SkipResult {
      nextline: Some((curidx, curline)),
      nextline_kw: curkw,
      skip_end: previdx,
    }
  }
}

#[cfg(test)]
mod tests {
  use card::ges::GesType::GesNode;
  use card::keyword::Keyword;
  use nocommentiter::CommentLess;
  use skipresult::SkipLine;

  const COMMENTS: [&'static str; 8] = [
    "#This", "#is", "#an", "#example", "of", "some", "lines", ".",
  ];

  #[test]
  fn nocommentiter_works_with_slice() {
    let mut li = COMMENTS.iter().enumerate().remove_comments();
    assert_eq!(li.next(), Some((4, &COMMENTS[4])));
    assert_eq!(li.next(), Some((5, &COMMENTS[5])));
  }

  #[test]
  fn linesiter_works_with_vec() {
    let v: Vec<String> = vec!["abc".to_owned(), "abc".to_owned()];

    let mut li = v.iter().enumerate().remove_comments();
    assert_eq!(li.next(), Some((0, &v[0])));
    assert_eq!(li.next(), Some((1, &v[1])));
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
    let mut li = COMMENTS.iter().skip(4).enumerate().remove_comments();
    assert_eq!(li.skip_to_next_keyword(), None);
  }

  #[test]
  fn linesiter_finds_real_keywords() {
    let mut li = KEYWORD_LINES.iter().enumerate().remove_comments();
    {
      assert_eq!(
        li.skip_to_next_keyword().unwrap().line,
        (2, &KEYWORD_LINES[2])
      );
      assert_eq!(
        li.skip_to_next_keyword().unwrap().line,
        (4, &KEYWORD_LINES[4])
      );
      assert_eq!(li.skip_to_next_keyword(), None);
    }
    assert_eq!(li.next(), None);
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
    let mut li = GES1.iter().enumerate().remove_comments();

    assert_eq!(li.skip_ges(GesNode).nextline, Some((4, &GES1[4])));
    assert_eq!(li.next(), None);
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
    let mut li = GES2.iter().enumerate().remove_comments();

    assert_eq!(li.skip_ges(GesNode).nextline, Some((3, &GES2[3])));
    assert_eq!(li.skip_ges(GesNode).nextline, None);
    assert_eq!(li.next(), None);
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
    let mut li = GES3.iter().enumerate().remove_comments();

    assert_eq!(li.skip_ges(GesNode).nextline, Some((2, &GES3[2])));
    assert_eq!(li.skip_ges(GesNode).nextline, Some((7, &GES3[7])));
    assert_eq!(li.next(), Some((8, &GES3[8])));
  }

  const GES4: [&'static str; 2] = ["wupdiwup", "NODE  / "];

  #[test]
  fn ges_can_skip_nothing() {
    let mut li = GES4.iter().enumerate().remove_comments();

    assert_eq!(li.skip_ges(GesNode).nextline, Some((0, &GES4[0])));
    assert_eq!(li.next(), Some((1, &GES4[1])));
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
    let mut li = GES6.iter().enumerate().remove_comments();

    let tmp = li.skip_ges(GesNode);
    assert_eq!(tmp.nextline, Some((6, &GES6[6])));
    assert_eq!(tmp.skip_end, Some(4));
    assert_eq!(li.next(), None);
  }

  const GES7: [&'static str; 4] = [
    "#        PART 1234",
    "#Comment here",
    "$Another comment",
    "#NODE  / ",
  ];

  #[test]
  fn ges_works_with_only_comments() {
    let mut li = GES7.iter().enumerate().remove_comments();

    let tmp = li.skip_ges(GesNode);
    assert_eq!(tmp.nextline, None);
    assert_eq!(tmp.skip_end, None);
    assert_eq!(li.next(), None);
  }

  const GES8: [&'static str; 4] = [
    "        PART 1234",
    "#Comment here",
    "$Another comment",
    "#NODE  / ",
  ];

  #[test]
  fn ges_skips_over_comments_after_end() {
    let mut li = GES8.iter().enumerate().remove_comments();

    let tmp = li.skip_ges(GesNode);
    assert_eq!(tmp.nextline, None);
    assert_eq!(tmp.skip_end, Some(0));
    assert_eq!(li.next(), None);
  }

  const CARD_NSMAS: [&'static str; 7] = [
    "NSMAS /        1              0.                                                ",
    "$#                                                                         TITLE",
    "NAME NSMAS / ->1                                                                ",
    "        ELE 123",
    "        PART 2345",
    "        END",
    "#Comment",
  ];

  #[test]
  fn itr_skips_nsmas() {
    let mut li = CARD_NSMAS.iter().enumerate().remove_comments();
    let firstline = li.next().unwrap();
    let kw = Keyword::parse(&firstline.1);
    let sr = SkipLine {
      line: firstline,
      line_kw: kw.unwrap(),
    };

    let tmp = li.skip_card(&sr);
    assert_eq!(tmp.nextline, None);
    assert_eq!(tmp.skip_end, Some(5));
  }

  const CARD_NODES: [&'static str; 9] = [
    "NODE  /       28     30.29999924            50.5              0.",
    "NODE  /       28     30.29999924            50.5              0.",
    "NODE  /       28     30.29999924            50.5              0.",
    "#COMMENT",
    "NODE  /       28     30.29999924            50.5              0.",
    "$COMMENT",
    "NODE  /       28     30.29999924            50.5              0.",
    "NODE  /       28     30.29999924            50.5              0.",
    "SHELL /     ",
  ];

  #[test]
  fn itr_skips_nodes() {
    let mut li = CARD_NODES.iter().enumerate().remove_comments();
    let firstline = li.next().unwrap();
    let kw = Keyword::parse(&firstline.1);
    let sr = SkipLine {
      line: firstline,
      line_kw: kw.unwrap(),
    };

    let tmp = li.skip_card_gather(&sr);
    assert_eq!(tmp.nextline, Some((8, &"SHELL /     ")));
    assert_eq!(tmp.skip_end, Some(7));
  }

  const CARD_MASS_INCOMPLETE: [&'static str; 9] = [
    "$ MASS Card",
    "$#         IDNOD    IFRA   Blank            DISr            DISs            DISt",
    "MASS  /        0       0                                                        ",
    "$#                                                                         TITLE",
    "NAME MASS  / ->1                                                                ",
    "$# BLANK              Mx              My              Mz",
    "$# BLANK              Ix              Iy              Iz                   Blank",
    "NODE  /      ",
    "                                                        ",
  ];

  #[test]
  fn itr_skips_incomplete_cards() {
    let mut li = CARD_MASS_INCOMPLETE
      .iter()
      .enumerate()
      .skip(2)
      .remove_comments();
    let firstline = li.next().unwrap();
    let kw = Keyword::parse(&firstline.1);
    let sr = SkipLine {
      line: firstline,
      line_kw: kw.unwrap(),
    };

    let tmp = li.skip_card(&sr);
    assert_eq!(tmp.nextline, Some((7, &"NODE  /      ")));
    assert_eq!(tmp.skip_end, Some(4));
  }

  const CARD_MASS_OPT: [&'static str; 12] = [
    "MASS  /        0       0                                                        ",
    "$#                                                                         TITLE",
    "NAME MASS  / ->1                                                                ",
    "$# BLANK              Mx              My              Mz",
    "                                                        ",
    "$# BLANK              Ix              Iy              Iz                   Blank",
    "                                                                                &",
    "                                                  ",
    "        PART 1234",
    "        GRP 'nogrp'",
    "        END",
    "$Comment",
  ];

  #[test]
  fn itr_skips_optional_lines() {
    let mut li = CARD_MASS_OPT.iter().enumerate().remove_comments();
    let firstline = li.next().unwrap();
    let kw = Keyword::parse(&firstline.1);
    let sr = SkipLine {
      line: firstline,
      line_kw: kw.unwrap(),
    };

    let tmp = li.skip_card(&sr);
    assert_eq!(tmp.nextline, None);
    assert_eq!(tmp.skip_end, Some(10));
  }

  const LINES_GATHER: [&'static str; 20] = [
    /* 0 */
    "NODE  /        1              0.             0.5              0.",
    /* 1 */
    "NODE  /        1              0.             0.5              0.",
    /* 2 */
    "NODE  /        1              0.             0.5              0.",
    /* 3 */
    "NODE  /        1              0.             0.5              0.",
    /* 4 */
    "#Comment here",
    /* 5 */
    "SHELL /     3129       1       1    2967    2971    2970",
    /* 6 */
    "invalid line here",
    /* 7 */
    "SHELL /     3129       1       1    2967    2971    2970",
    /* 8 */
    "SHELL /     3129       1       1    2967    2971    2970",
    /* 9 */
    "#Comment",
    /* 10 */
    "#Comment",
    /* 11 */
    "SHELL /     3129       1 rust hint unsafe unreachable      1    2967    2971    2970",
    /* 12 */
    "SHELL /     3129       1       1    2967    2971    2970",
    /* 13 */
    "$Comment",
    /* 14 */
    "SHELL /     3129       1       1    2967    2971    2970",
    /* 15 */
    "SHELL /     3129       1       1    2967    2971    2970",
    /* 16 */
    "$Comment",
    /* 17 */
    "#Comment",
    /* 18 */
    "NODE  /        1              0.             0.5              0.",
    /* 19 */
    "NODE  /        1              0.             0.5              0.",
  ];

  #[test]
  fn itr_skips_gather_cards() {
    let mut li = LINES_GATHER.iter().enumerate().remove_comments();
    let firstline = li.next().unwrap();
    let kw = Keyword::parse(&firstline.1);
    let sr = SkipLine {
      line: firstline,
      line_kw: kw.unwrap(),
    };

    let mut tmp = li.skip_fold(&sr);
    assert_eq!(tmp.nextline, Some((5, &LINES_GATHER[5])));
    assert_eq!(tmp.skip_end, Some(3));

    tmp = li.skip_fold(&SkipLine {
      line: tmp.nextline.unwrap(),
      line_kw: tmp.nextline_kw.unwrap(),
    });
    assert_eq!(tmp.nextline, Some((6, &LINES_GATHER[6])));
    assert_eq!(tmp.skip_end, Some(5));

    let skipped = li.skip_to_next_keyword().unwrap();
    tmp = li.skip_fold(&skipped);
    assert_eq!(tmp.nextline, Some((18, &LINES_GATHER[18])));
    assert_eq!(tmp.skip_end, Some(15));

    tmp = li.skip_fold(&SkipLine {
      line: tmp.nextline.unwrap(),
      line_kw: tmp.nextline_kw.unwrap(),
    });
    assert_eq!(tmp.nextline, None);
    assert_eq!(tmp.skip_end, None);
  }

}
