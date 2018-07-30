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
use skipresult::{KeywordLine, ParsedLine, SkipResult};

// Used in skip functions. Returns the next `ParsedLine` from the iterator. If
// theres no next line, return a `SkipResult` containing the line number of
// `prevline` and nothing else.
macro_rules! next_or_return_previdx {
  ($self:ident, $prevline:ident) => {
    match $self.next() {
      None => {
        return SkipResult {
          skip_end: $prevline.map(|p: ParsedLine<'a, T>| p.number),
          ..Default::default()
        }
      }
      Some(t) => t,
    };
  };
}

// Used in skip_ges to get the next line. If it's None, we're at the end of
// the file and only return what we found before. Also used in `advance_some!`
macro_rules! next_or_return_some_previdx {
  ($self:ident, $prevline:ident) => {
    match $self.next() {
      None => {
        return Some(SkipResult {
          skip_end: $prevline.map(|p: ParsedLine<'a, T>| p.number),
          ..Default::default()
        })
      }
      Some(t) => t,
    };
  };
}

// In the same veins as above, get the next line from the iterator, or return
// None from the function.
macro_rules! next_or_return_none {
  ($self:ident) => {
    match $self.next() {
      None => return None,
      Some(t) => t,
    };
  };
}

// A common pattern for nocommentiter: Save Some(nextline) in prevline,
// and advance the iterator. Save in nextline, or return a SkipResult built
// from prevline's line number
macro_rules! advance {
  ($self:ident, $prevline:ident, $nextline:ident) => {
    $prevline = Some($nextline);
    $nextline = next_or_return_previdx!($self, $prevline);
  };
}

// Same as advance above, just that the `SkipResult` is wrapped in `Some`. Used
// in skip_ges.
macro_rules! advance_some {
  ($self:ident, $prevline:ident, $nextline:ident) => {
    $prevline = Some($nextline);
    $nextline = next_or_return_some_previdx!($self, $prevline);
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
  type Item = ParsedLine<'a, T>;

  fn next(&mut self) -> Option<Self::Item> {
    while let Some((i, n)) = self.it.next() {
      let t = n.as_ref().as_bytes();
      if !(t.len() > 0 && (t[0] == b'#' || t[0] == b'$')) {
        return Some(ParsedLine {
          number: i,
          text: n,
          keyword: Keyword::parse(n),
        });
      }
    }
    None
  }
}

impl<'a, I, T: 'a> CommentLess for I
where
  I: Iterator<Item = (usize, &'a T)>,
  T: AsRef<str>,
{
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
  pub fn skip_to_next_keyword<'b>(&'b mut self) -> Option<KeywordLine<'a, T>> {
    let mut line = None;

    while line.is_none() {
      line = next_or_return_none!(self).try_into_keywordline();
    }

    line
  }

  /// Advance the iterator until the first line after a General Entity
  /// Selection (GES).
  ///
  /// Returns `None` if skipline neither ends the GES, nor is
  /// contained in it. We did not try to advance the iterator in this case.
  /// Returns `Some(Default::default())` if `skipline` ends the GES, and the
  /// file ends after that.
  pub fn skip_ges<'b>(
    &'b mut self,
    ges: GesType,
    skipline: &ParsedLine<'a, T>,
  ) -> Option<SkipResult<'a, T>> {
    let mut prevline: Option<ParsedLine<'a, T>> = None;
    let mut nextline: ParsedLine<'a, T>;

    let contained = ges.contains(skipline.text);
    let ends = ges.ended_by(skipline.text);

    if ends {
      nextline = next_or_return_some_previdx!(self, prevline);
      Some(SkipResult {
        nextline: Some(nextline),
        skip_end: Some(skipline.number),
      })
    } else if !ends && !contained {
      None
    } else {
      // Need to save this in case the iterator ends in the next line
      prevline = Some(ParsedLine {
        number: skipline.number,
        text: skipline.text,
        keyword: skipline.keyword,
      });
      nextline = next_or_return_some_previdx!(self, prevline);

      while ges.contains(nextline.text) {
        advance_some!(self, prevline, nextline);
      }

      if ges.ended_by(nextline.text) {
        advance_some!(self, prevline, nextline);
      }

      Some(SkipResult {
        nextline: Some(nextline),
        skip_end: prevline.map(|p| p.number),
      })
    }
  }

  /// A wrapper around [`skip_card`](NoCommentIter::skip_card) and
  /// [`skip_card_gather`](NoCommentIter::skip_card_gather), dispatching by
  /// value of [`Card.ownfold`](::card::Card)
  pub fn skip_fold<'b>(
    &'b mut self,
    skipline: &KeywordLine<'a, T>,
  ) -> SkipResult<'a, T> {
    let card: &Card = (&skipline.keyword).into();

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
    skipline: &KeywordLine<'a, T>,
  ) -> SkipResult<'a, T> {
    let mut conds: Vec<CondResult> = vec![]; // the vec to hold the conditionals
    let mut cardlines = <&Card>::from(&skipline.keyword).lines.iter();
    let cardline = cardlines.next().unwrap_or_else(|| unreachable!());

    if let Line::Provides(_s, ref c) = cardline {
      conds.push(c.evaluate(skipline.text));
    }

    let mut prevline: Option<ParsedLine<'a, T>> = None;
    let mut nextline = next_or_return_previdx!(self, prevline);

    for cardline in cardlines {
      if nextline.keyword.is_some() {
        break;
      }

      match *cardline {
        Line::Provides(_s, ref c) => {
          conds.push(c.evaluate(&nextline.text));
          advance!(self, prevline, nextline);
        }
        Line::Ges(ref g) => {
          let tmp = self.skip_ges(*g, &nextline);

          match tmp {
            None => continue,
            Some(sr) => match sr.nextline {
              None => {
                return SkipResult {
                  nextline: None,
                  skip_end: sr.skip_end,
                }
              }
              Some(pl) => {
                prevline = Some(nextline);
                nextline = pl;
              }
            },
          }
        }
        Line::Cells(_s) => {
          advance!(self, prevline, nextline);
        }
        Line::Optional(_s, i) => {
          if conds.get(i as usize) == Some(&CondResult::Bool(true)) {
            advance!(self, prevline, nextline);
          } else {
            continue;
          }
        }
        Line::Repeat(_s, i) => {
          let num = match conds.get(i as usize) {
            Some(CondResult::Number(Some(u))) if *u > 0 => u,
            _ => continue,
          };

          // We need one more loop than *num because we need to get the next
          // line for the next outer iteration
          for _ in 0..*num {
            advance!(self, prevline, nextline);

            if nextline.keyword.is_some() {
              break;
            }
          }
        }
        Line::Block(_l, s) => loop {
          while !nextline.text.as_ref().starts_with(s) {
            advance!(self, prevline, nextline);

            if nextline.keyword.is_some() {
              break;
            }
          }
          advance!(self, prevline, nextline);
        },
        Line::OptionalBlock(s1, s2) => {
          if !nextline.text.as_ref().starts_with(s1) {
            continue;
          }
          while !nextline.text.as_ref().starts_with(s2) {
            advance!(self, prevline, nextline);

            if nextline.keyword.is_some() {
              break;
            }
          }
        }
      }
    }
    SkipResult {
      nextline: Some(nextline),
      skip_end: prevline.map(|p| p.number),
    }
  }

  /// Let [`NoCommentIter`](NoCommentIter) skip all given
  /// [`Card`](::card::Card)s, until the next card starts. The basic assumption
  /// is that the last line the iterator returned is a the first line of a card
  /// of the given type, but that might not always be strictly neccessary.
  pub fn skip_card_gather<'b>(
    &'b mut self,
    nextline: &KeywordLine<'a, T>,
  ) -> SkipResult<'a, T> {
    let mut curkw;
    let mut res;
    let mut curidx;
    let mut curline;
    let mut previdx = None;

    let card: &Card = (&nextline.keyword).into();

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
        Some(p) => {
          curkw = p.keyword;
          curline = p.text;
          curidx = p.number;
          // TODO: check this -1
          previdx = previdx.or_else(|| Some(curidx - 1));
        }
      }
      if curkw != Some(card.keyword) {
        break;
      } else {
        previdx = Some(curidx);
      }

      res = self.skip_card(&KeywordLine {
        number: curidx,
        text: curline,
        keyword: curkw.unwrap(),
      });
    }

    SkipResult {
      nextline: Some(ParsedLine {
        number: curidx,
        text: curline,
        keyword: curkw,
      }),
      skip_end: previdx,
    }
  }
}

#[cfg(test)]
mod tests {
  use card::ges::GesType::GesNode;
  use card::keyword::Keyword::*;
  use nocommentiter::CommentLess;
  use skipresult::{KeywordLine, ParsedLine};

  macro_rules! pline {
    ($number:expr, $text:expr, $keyword:expr) => {
      ParsedLine {
        number: $number,
        text: $text,
        keyword: $keyword,
      }
    };
  }

  macro_rules! kwline {
    ($number:expr, $text:expr, $keyword:expr) => {
      KeywordLine {
        number: $number,
        text: $text,
        keyword: $keyword,
      }
    };
  }

  const COMMENTS: [&'static str; 8] = [
    "#This", "#is", "#an", "#example", "of", "some", "lines", ".",
  ];

  #[test]
  fn nocommentiter_works_with_slice() {
    let mut li = COMMENTS.iter().enumerate().remove_comments();
    assert_eq!(li.next().unwrap(), pline!(4, &COMMENTS[4], None));
    assert_eq!(li.next().unwrap(), pline!(5, &COMMENTS[5], None));
  }

  #[test]
  fn linesiter_works_with_vec() {
    let v: Vec<String> = vec!["abc".to_owned(), "abc".to_owned()];

    let mut li = v.iter().enumerate().remove_comments();
    assert_eq!(li.next().unwrap(), pline!(0, &v[0], None));
    assert_eq!(li.next().unwrap(), pline!(1, &v[1], None));
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
        li.skip_to_next_keyword().unwrap(),
        kwline!(2, &KEYWORD_LINES[2], Node)
      );
      assert_eq!(
        li.skip_to_next_keyword().unwrap(),
        kwline!(4, &KEYWORD_LINES[4], Nsmas)
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
    let nextline = li.next().unwrap();
    let tmp = li.skip_ges(GesNode, &nextline).unwrap();

    assert_eq!(tmp.nextline.unwrap(), pline!(4, &GES1[4], Some(Node)));
    assert_eq!(tmp.skip_end, Some(3));
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

    let nextline = li.next().unwrap();
    let tmp = li.skip_ges(GesNode, &nextline).unwrap();
    assert_eq!(tmp.nextline.unwrap(), pline!(3, &GES2[3], None));
    assert_eq!(tmp.skip_end, Some(2));

    let nextline = li.next().unwrap();
    let tmp = li.skip_ges(GesNode, &nextline).unwrap();
    assert_eq!(tmp.nextline, None);
    assert_eq!(tmp.skip_end, Some(8));
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

    let nextline = li.next().unwrap();
    let mut tmp = li.skip_ges(GesNode, &nextline).unwrap();
    assert_eq!(tmp.nextline.unwrap(), pline!(2, &GES3[2], Some(Node)));
    assert_eq!(tmp.skip_end, Some(1));

    let nextline = li.next().unwrap();
    tmp = li.skip_ges(GesNode, &nextline).unwrap();
    assert_eq!(tmp.nextline.unwrap(), pline!(7, &GES3[7], None));
    assert_eq!(tmp.skip_end, Some(6));

    assert_eq!(li.next().unwrap(), pline!(8, &GES3[8], None));
  }

  const GES4: [&'static str; 2] = ["wupdiwup", "NODE  / "];

  #[test]
  fn ges_can_skip_nothing() {
    let mut li = GES4.iter().enumerate().remove_comments();

    let nextline = li.next().unwrap();
    let tmp = li.skip_ges(GesNode, &nextline);
    assert!(tmp.is_none());
    assert_eq!(li.next().unwrap(), pline!(1, &GES4[1], Some(Node)));
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

    let nextline = li.next().unwrap();
    let tmp = li.skip_ges(GesNode, &nextline).unwrap();
    assert_eq!(tmp.nextline.unwrap(), pline!(6, &GES6[6], Some(Node)));
    assert_eq!(tmp.skip_end, Some(4));

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

    let nextline = li.next().unwrap();
    let tmp = li.skip_ges(GesNode, &nextline).unwrap();
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

    let tmp = li.skip_card(&firstline.try_into_keywordline().unwrap());
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

    let tmp = li.skip_card_gather(&firstline.try_into_keywordline().unwrap());
    assert_eq!(
      tmp.nextline.unwrap(),
      pline!(8, &"SHELL /     ", Some(Shell))
    );
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

    let tmp = li.skip_card(&firstline.try_into_keywordline().unwrap());
    assert_eq!(
      tmp.nextline.unwrap(),
      pline!(7, &"NODE  /      ", Some(Node))
    );
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

    let tmp = li.skip_card(&firstline.try_into_keywordline().unwrap());
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

    let mut tmp = li.skip_fold(&firstline.try_into_keywordline().unwrap());
    let mut tmp_nextline = tmp.nextline.unwrap();
    assert_eq!(tmp_nextline, pline!(5, &LINES_GATHER[5], Some(Shell)));
    assert_eq!(tmp.skip_end, Some(3));

    tmp = li.skip_fold(&tmp_nextline.try_into_keywordline().unwrap());
    tmp_nextline = tmp.nextline.unwrap();
    assert_eq!(tmp_nextline, pline!(6, &LINES_GATHER[6], None));
    assert_eq!(tmp.skip_end, Some(5));

    let skipped = li.skip_to_next_keyword().unwrap();
    tmp = li.skip_fold(&skipped.into());
    tmp_nextline = tmp.nextline.unwrap();
    assert_eq!(tmp_nextline, pline!(18, &LINES_GATHER[18], Some(Node)));
    assert_eq!(tmp.skip_end, Some(15));

    tmp = li.skip_fold(&tmp_nextline.try_into_keywordline().unwrap());
    assert_eq!(tmp.nextline, None);
    assert_eq!(tmp.skip_end, None);
  }

}
