//! This module holds [`LinesIter`](crate::linesiter::LinesIter), the
//! central datastructure to parse the lines of a buffer.
//!
//! It returns enumerated Lines, but skips Comments (lines starting with `$` or
//! `#`). All skip functions, used by
//! [`parse_from_iter`](crate::bufdata::BufData::parse_from_iter), work on a
//! [`LinesIter`](crate::linesiter::LinesIter).
use crate::{
  bufdata::highlights::Highlights,
  card::{
    ges::GesType,
    line::{CondResult, Line as CardLine},
    Card,
  },
  linenr::LineNr,
  lines::{KeywordLine, ParsedLine},
  skipresult::SkipResult,
};

// Used in skip functions. Returns the next `ParsedLine` from the iterator. If
// theres no next line, return a `SkipResult` containing the line number of
// `prevline` and nothing else.
macro_rules! next_or_return_previdx {
  ($self:ident, $previdx:expr) => {
    match $self.next() {
      None => {
        return SkipResult {
          skip_end: $previdx,
          nextline: None,
        };
      }
      Some(t) => t,
    };
  };
}

// Used in skip_ges to get the next line. If it's None, we're at the end of
// the file and only return what we found before. Also used in `advance_some!`
macro_rules! next_or_return_some_previdx {
  ($self:ident, $previdx:expr) => {
    match $self.next() {
      None => {
        return Some(SkipResult {
          skip_end: $previdx,
          nextline: None,
        });
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
  ($self:ident, $previdx:ident, $nextline:ident) => {
    $previdx = $nextline.number;
    $nextline = next_or_return_previdx!($self, $previdx);
  };
}

// Same as advance above, just that the `SkipResult` is wrapped in `Some`. Used
// in skip_ges.
macro_rules! advance_some {
  ($self:ident, $previdx:ident, $nextline:ident) => {
    $previdx = $nextline.number;
    $nextline = next_or_return_some_previdx!($self, $previdx);
  };
}

// Check if $nextline has number $line or higher, and returns $cardline_opt or
// None as appropriate. Otherwise, fetches the next element of $self, or returns
// None
//
// Used in get_cardline
macro_rules! return_idx_or_next {
  ($self:ident, $nextline:ident, $line:expr, $idx_opt:expr) => {
    // 2nd cond can be true if we're inside a ges, the loop will step over it
    // and return the line after it
    if $nextline.keyword.is_some() || $nextline.number > $line {
      return None;
    } else if $nextline.number == $line {
      return $idx_opt;
    } else {
      match $self.next() {
        None => return None,
        Some(t) => t,
      }
    };
  };
}
/// The struct simply holds a type instance. Skipping comments is done in the
/// Iterator implementation.
pub struct LinesIter<'a, I>
where
  I: Iterator<Item = &'a ParsedLine<'a>>,
{
  it: I,
}

impl<'a, I> Iterator for LinesIter<'a, I>
where
  I: Iterator<Item = &'a ParsedLine<'a>>,
{
  type Item = &'a ParsedLine<'a>;

  fn next(&mut self) -> Option<Self::Item> {
    self.it.next()
  }
}

impl<'a, I> LinesIter<'a, I>
where
  I: Iterator<Item = &'a ParsedLine<'a>>,
{
  pub fn new(it: I) -> Self {
    Self { it }
  }

  /// Advance the iterator until meeting the first line with a keyword. If the
  /// file ends before that, return `None`.
  pub fn skip_to_next_keyword<'b>(&'b mut self) -> Option<KeywordLine<'a>> {
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
  /// contained in it. We do not try to advance the iterator in this case.
  pub fn skip_ges<'b>(
    &'b mut self,
    ges: GesType,
    skipline: &ParsedLine<'a>,
  ) -> Option<SkipResult<'a>> {
    let mut previdx: LineNr = skipline.number;
    let mut nextline: &'a ParsedLine<'a>;

    let contained = ges.contains(skipline.text.as_ref());
    let ends = ges.ended_by(skipline.text.as_ref());

    if ends {
      nextline = next_or_return_some_previdx!(self, previdx);
      Some(SkipResult {
        nextline: Some(nextline),
        skip_end: previdx,
      })
    } else if !ends && !contained {
      None
    } else {
      nextline = next_or_return_some_previdx!(self, skipline.number);

      while ges.contains(nextline.text.as_ref()) {
        advance_some!(self, previdx, nextline);
      }

      if ges.ended_by(nextline.text.as_ref()) {
        advance_some!(self, previdx, nextline);
      }

      Some(SkipResult {
        nextline: Some(nextline),
        skip_end: previdx,
      })
    }
  }

  /// A wrapper around [`skip_card`](LinesIter::skip_card) and
  /// [`skip_card_gather`](LinesIter::skip_card_gather), dispatching by value of
  /// [`Card.ownfold`](crate::card::Card::ownfold)
  pub fn skip_fold<'b>(
    &'b mut self,
    skipline: &KeywordLine<'a>,
    highlights: &mut Highlights,
  ) -> SkipResult<'a> {
    let card: &Card = (&skipline.keyword).into();

    if card.ownfold {
      self.skip_card(&skipline, card, highlights)
    } else {
      self.skip_card_gather(&skipline, card, highlights)
    }
  }

  /// Let [`NoCommentIter`](NoCommentIter) skip the given
  /// [`Card`](::card::Card), but only skip this 1 card. This only really makes
  /// sense when the last line the iterator returned is the line with the
  /// keyword starting that card, which is passed as `skipline`.
  ///
  /// If you want to skip all cards of a given type, use
  /// [`skip_card_gather`](NoCommentIter::skip_card_gather)
  fn skip_card<'b>(
    &'b mut self,
    skipline: &KeywordLine<'a>,
    card: &Card,
    highlights: &mut Highlights,
  ) -> SkipResult<'a> {
    let mut conds: Vec<CondResult> = vec![]; // the vec to hold the conditionals
    let mut cardlines = card.lines.iter();
    let cardline = cardlines.next().unwrap_or_else(|| unreachable!());

    if let CardLine::Provides(_s, ref c) = cardline {
      conds.push(c.evaluate(skipline.text));
    }

    highlights.add_line_highlights(skipline.number, skipline.text, cardline);

    let mut previdx: LineNr = skipline.number;
    let mut nextline = next_or_return_previdx!(self, previdx);

    for cardline in cardlines {
      if nextline.keyword.is_some() {
        break;
      }

      match *cardline {
        CardLine::Provides(_s, ref c) => {
          conds.push(c.evaluate(nextline.text.as_ref()));
          advance!(self, previdx, nextline);
        }
        CardLine::Ges(ref g) => {
          if let Some(sr) = self.skip_ges(*g, nextline) {
            match sr.nextline {
              None => return sr,
              Some(pl) => {
                previdx = sr.skip_end;
                nextline = pl;
              }
            };
          }
        }
        CardLine::Cells(_s) => {
          highlights.add_line_highlights(
            nextline.number,
            nextline.text.as_ref(),
            cardline,
          );

          advance!(self, previdx, nextline);
        }
        CardLine::Optional(_s, i) => {
          if conds.get(i as usize) == Some(&CondResult::Bool(true)) {
            advance!(self, previdx, nextline);
          } else {
            continue;
          }
        }
        CardLine::Repeat(_s, i) => {
          let num = match conds.get(i as usize) {
            Some(CondResult::Number(Some(u))) if *u > 0 => u,
            _ => continue,
          };

          // TODO(KillTheMule): Is this comment still right? Guess not...
          // We need one more loop than *num because we need to get the next
          // line for the next outer iteration
          for _ in 0..*num {
            advance!(self, previdx, nextline);

            if nextline.keyword.is_some() {
              break;
            }
          }
        }
        CardLine::Block(_l, s) => loop {
          while !nextline.text.as_ref().starts_with(s) {
            advance!(self, previdx, nextline);

            if nextline.keyword.is_some() {
              break;
            }
          }
          advance!(self, previdx, nextline);
        },
        CardLine::OptionalBlock(s1, s2) => {
          if !nextline.text.as_ref().starts_with(s1) {
            continue;
          }
          while !nextline.text.as_ref().starts_with(s2) {
            advance!(self, previdx, nextline);

            if nextline.keyword.is_some() {
              break;
            }
          }
        }
      }
    }
    SkipResult {
      nextline: Some(nextline),
      skip_end: previdx,
    }
  }

  /// Let [`NoCommentIter`](NoCommentIter) skip all given
  /// [`Card`](::card::Card)s, until the next different card starts. The basic
  /// assumption is that the last line the iterator returned is a the first line
  /// of a card of the given type, which is passed as `skipline`.
  fn skip_card_gather<'b>(
    &'b mut self,
    skipline: &KeywordLine<'a>,
    card: &Card,
    highlights: &mut Highlights,
  ) -> SkipResult<'a> {
    let mut r = self.skip_card(&skipline, card, highlights);

    while let Some(p) = r.nextline {
      if let Some(kl) = p.try_into_keywordline() {
        if kl.keyword == card.keyword() {
          r = self.skip_card(&kl, card, highlights);
        } else {
          break;
        }
      } else {
        // Happens on invalid lines
        break;
      }
    }

    r
  }

  /// Return the index of a `CardlineHint` [crate::card::CardlineHint]
  /// corresponding to the given `LineNr`[crate::linenr::LineNr]. Return `None`
  /// if the card does not contain a line with this number, or the iterator ends
  /// before we reached it.
  ///
  /// Assumes that `cline` is  the first line of the card, and the iterator
  /// starts after that.
  pub fn get_cardline_hints_index(
    mut self,
    cline: &KeywordLine<'a>,
    card: &'static Card,
    line: LineNr,
  ) -> Option<u8> {
    let mut cardlines = card.lines.iter();
    let cardline = cardlines.next().unwrap_or_else(|| unreachable!());

    if cline.number == line {
      return Some(0);
    }

    // need to track manually
    let mut idx = 0;
    // the vec to hold the conditionals
    let mut conds: Vec<CondResult> = vec![];

    if let CardLine::Provides(_s, ref c) = cardline {
      conds.push(c.evaluate(cline.text));
    }

    let mut nextline = next_or_return_none!(self);

    for cardline in cardlines {
      idx += 1;

      match *cardline {
        CardLine::Provides(_s, ref c) => {
          conds.push(c.evaluate(nextline.text.as_ref()));
          nextline = return_idx_or_next!(self, nextline, line, Some(idx));
        }
        CardLine::Ges(ref g) => {
          if let Some(sr) = self.skip_ges(*g, nextline) {
            match sr.nextline {
              None => return None,
              Some(pl) => {
                // we skipped over the line in question
                if pl.number > line {
                  return Some(idx);
                }
                nextline = pl;
              }
            };
          }
        }
        CardLine::Cells(_s) => {
          nextline = return_idx_or_next!(self, nextline, line, Some(idx));
        }
        CardLine::Optional(_s, i) => {
          if conds.get(i as usize) == Some(&CondResult::Bool(true)) {
            nextline = return_idx_or_next!(self, nextline, line, Some(idx));
          } else {
            continue;
          }
        }
        CardLine::Repeat(_s, i) => {
          let num = match conds.get(i as usize) {
            Some(CondResult::Number(Some(u))) if *u > 0 => u,
            _ => continue,
          };

          for _ in 0..*num {
            nextline = return_idx_or_next!(self, nextline, line, Some(idx));
          }
        }
        CardLine::Block(l, s) => loop {
          debug_assert!(l.len() < u8::max_value() as usize - idx as usize);
          while !nextline.text.as_ref().starts_with(s) {
            for i in 0..l.len() {
              nextline =
                return_idx_or_next!(self, nextline, line, Some(idx + i as u8));
            }
          }
          idx += l.len() as u8;

          // this should be the line that contains the block delimiting string
          if nextline.number == line {
            return Some(idx);
          }
        },
        CardLine::OptionalBlock(s1, s2) => {
          if !nextline.text.as_ref().starts_with(s1) {
            continue;
          }
          while !nextline.text.as_ref().starts_with(s2) {
            nextline = return_idx_or_next!(self, nextline, line, Some(idx));
            idx += 1;
          }

          // this should be the line that contains the block delimiting string
          if nextline.number == line {
            return Some(idx);
          }
        }
      }
    }
    None
  }
}

#[cfg(test)]
mod tests {
  use crate::{
    bufdata::highlights::Highlights,
    card::{ges::GesType::GesNode, keyword::Keyword::*, Card},
    carddata::*,
    linenr::LineNr,
    lines::{KeywordLine, Lines, ParsedLine, RawLine::*},
  };

  macro_rules! pline {
    ($number:expr, $text:expr, $keyword:expr) => {
      ParsedLine {
        number: $number,
        text: OriginalLine($text.as_ref()),
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

  const COMMENTS: &'static str = "#This\n$is\n#an\n#example\nof\nsome\
                                  \nlines\n.";

  #[test]
  fn works_with_slice() {
    let mut lines = Lines::new();
    lines.parse_slice(COMMENTS.as_ref());
    let mut l = lines.iter();
    assert_eq!(l.next(), Some(&pline!(4.into(), "of", None)));
    assert_eq!(l.next(), Some(&pline!(5.into(), "some", None)));
  }

  const NOKEYWORD_LINES: &'static str = "\nsome\nlines\n.";

  #[test]
  fn needs_no_keywords() {
    let mut lines = Lines::new();
    lines.parse_slice(NOKEYWORD_LINES.as_ref());
    let mut l = lines.iter();
    assert_eq!(l.skip_to_next_keyword(), None);
  }

  const KEYWORD_LINES: &'static str = "#Comment\n   nokeyword\nNODE  / \
                                       \n#example\nNSMAS / \nsome\nlines\n.";

  #[test]
  fn finds_real_keywords() {
    let mut lines = Lines::new();
    lines.parse_slice(KEYWORD_LINES.as_ref());
    let mut l = lines.iter();
    assert_eq!(
      l.skip_to_next_keyword(),
      Some(kwline!(2.into(), b"NODE  / ", Node))
    );
    assert_eq!(
      l.skip_to_next_keyword(),
      Some(kwline!(4.into(), b"NSMAS / ", Nsmas))
    );
    assert_eq!(l.skip_to_next_keyword(), None);
    assert_eq!(l.next(), None);
  }

  const GES1: &'static str = "        PART 1234\
                              \n        OGRP 'hausbau'\
                              \n        DELGRP>NOD 'nix'\
                              \n        END\
                              \nNODE  / ";
  #[test]
  fn can_skip_ges() {
    let mut lines = Lines::new();
    lines.parse_slice(GES1.as_ref());
    let mut l = lines.iter();

    let nextline = l.next().unwrap();
    let tmp = l.skip_ges(GesNode, &nextline).unwrap();
    assert_eq!(
      tmp.nextline.unwrap(),
      &pline!(4.into(), b"NODE  / ", Some(Node))
    );
    assert_eq!(tmp.skip_end, 3.into());
    assert_eq!(l.next(), None);
  }

  const GES2: &'static str = "        PART 1234\
                              \n        OGRP 'hausbau'\
                              \n        END\
                              \n        DELGRP>NOD 'nix'\
                              \n        MOD 10234\
                              \n        NOD 1 23 093402 82\
                              \n        END_MOD\
                              \n        DELELE 12\
                              \n        END";

  const GES2_NEXT: &[u8] = b"        DELGRP>NOD 'nix'";

  #[test]
  fn can_skip_ges_repeatedly() {
    let mut lines = Lines::new();
    lines.parse_slice(GES2.as_ref());
    let mut l = lines.iter();

    let mut nextline = l.next().unwrap();
    let mut tmp = l.skip_ges(GesNode, &nextline).unwrap();
    assert_eq!(tmp.nextline.unwrap(), &pline!(3.into(), GES2_NEXT, None));
    assert_eq!(tmp.skip_end, 2.into());

    nextline = l.next().unwrap();
    tmp = l.skip_ges(GesNode, &nextline).unwrap();
    assert_eq!(tmp.nextline, None);
    assert_eq!(tmp.skip_end, 8.into());
    assert_eq!(l.next(), None);
  }

  const GES3: &'static str = "        PART 1234\
                              \n        OGRP 'hausbau'\
                              \nNODE  /         END\
                              \n        DELGRP>NOD 'nix'\
                              \n        MOD 10234\
                              \n        NOD 1 23 093402 82\
                              \n        END_MOD\
                              \nWhatever\
                              \n        END";

  const GES3_FIRST: &'static str = "NODE  /         END";
  const GES3_SECOND: &'static str = "Whatever";
  const GES3_LAST: &'static str = "        END";

  #[test]
  fn ends_ges_without_end() {
    let mut lines = Lines::new();
    lines.parse_slice(GES3.as_ref());
    let mut l = lines.iter();
    let mut nextline = l.next().unwrap();
    let mut tmp = l.skip_ges(GesNode, &nextline).unwrap();
    assert_eq!(
      tmp.nextline.unwrap(),
      &pline!(2.into(), GES3_FIRST, Some(Node))
    );
    assert_eq!(tmp.skip_end, 1.into());

    nextline = l.next().unwrap();
    tmp = l.skip_ges(GesNode, &nextline).unwrap();
    assert_eq!(tmp.nextline.unwrap(), &pline!(7.into(), GES3_SECOND, None));
    assert_eq!(tmp.skip_end, 6.into());
    assert_eq!(l.next(), Some(&pline!(8.into(), GES3_LAST, None)));
  }

  const GES4: &'static str = "wupdiwup\nNODE  / ";
  const GES4_LAST: &'static str = "NODE  / ";

  #[test]
  fn can_skip_empty_ges() {
    let mut lines = Lines::new();
    lines.parse_slice(GES4.as_ref());
    let mut l = lines.iter();
    let nextline = l.next().unwrap();
    let tmp = l.skip_ges(GesNode, &nextline);
    assert!(tmp.is_none());
    assert_eq!(l.next().unwrap(), &pline!(1.into(), GES4_LAST, Some(Node)));
  }

  const GES5: &'static str = "        PART 1234\
                              \n#Comment here\
                              \n        OGRP 'hausbau'\
                              \n        DELGRP>NOD 'nix'\
                              \n        END\
                              \n$Another comment\
                              \nNODE  / ";

  const GES5_NEXTL: &'static str = "NODE  / ";

  #[test]
  fn ges_works_with_comments() {
    let mut lines = Lines::new();
    lines.parse_slice(GES5.as_ref());
    let mut l = lines.iter();
    let nextline = l.next().unwrap();
    let tmp = l.skip_ges(GesNode, &nextline).unwrap();
    assert_eq!(
      tmp.nextline.unwrap(),
      &pline!(6.into(), GES5_NEXTL, Some(Node))
    );
    assert_eq!(tmp.skip_end, 4.into());
    assert_eq!(l.next(), None);
  }

  const GES6: &'static str = "        PART 1234\
                              \n#Comment here\
                              \n$Another comment\
                              \n#NODE  / ";

  #[test]
  fn ges_skips_comments_after_end() {
    let mut lines = Lines::new();
    lines.parse_slice(GES6.as_ref());
    let mut l = lines.iter();
    let nextline = l.next().unwrap();
    let tmp = l.skip_ges(GesNode, &nextline).unwrap();
    assert_eq!(tmp.nextline, None);
    assert_eq!(tmp.skip_end, 0.into());
    assert_eq!(l.next(), None);
  }

  const CARD_MASS_INCOMPLETE: &'static str =
    "$ MASS Card\
    \n$#         IDNOD    IFRA   Blank            DISr            DISs            DISt\
    \nMASS  /        0       0                                                        \
    \n$#                                                                         TITLE\
    \nNAME MASS  / ->1                                                                \
    \n$# BLANK              Mx              My              Mz\
    \n$# BLANK              Ix              Iy              Iz                   Blank\
    \nNODE  /      \
    \n                                                        ";

  #[test]
  fn skip_incomplete_cards() {
    let mut lines = Lines::new();
    lines.parse_slice(CARD_MASS_INCOMPLETE.as_ref());
    let mut l = lines.iter();
    let mut hls = Highlights::new();
    let firstline = l.next().unwrap();
    let tmp =
      l.skip_card(&firstline.try_into_keywordline().unwrap(), &MASS, &mut hls);
    assert_eq!(
      tmp.nextline.unwrap(),
      &pline!(7.into(), &"NODE  /      ", Some(Node))
    );
    assert_eq!(tmp.skip_end, 4.into());
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
    "SHELL /     3129       1       1    2967 2971    2970",
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
  fn skips_gather_cards() {
    let mut lines = Lines::new();
    let mut hls = Highlights::new();
    lines.parse_strs(&LINES_GATHER);
    let mut li = lines.iter();

    let firstline = li.next().unwrap();

    let mut tmp =
      li.skip_fold(&(firstline.try_into_keywordline()).unwrap(), &mut hls);
    let mut tmp_nextline = tmp.nextline.unwrap();
    assert_eq!(
      tmp_nextline,
      &pline!(5.into(), &LINES_GATHER[5], Some(Shell))
    );
    assert_eq!(tmp.skip_end, 3.into());

    tmp = li.skip_fold(&tmp_nextline.try_into_keywordline().unwrap(), &mut hls);
    tmp_nextline = tmp.nextline.unwrap();
    assert_eq!(tmp_nextline, &pline!(6.into(), &LINES_GATHER[6], None));
    assert_eq!(tmp.skip_end, 5.into());

    let skipped = li.skip_to_next_keyword().unwrap();
    tmp = li.skip_fold(&skipped.into(), &mut hls);
    tmp_nextline = tmp.nextline.unwrap();
    assert_eq!(
      tmp_nextline,
      &pline!(18.into(), &LINES_GATHER[18], Some(Node))
    );
    assert_eq!(tmp.skip_end, 15.into());

    tmp = li.skip_fold(&tmp_nextline.try_into_keywordline().unwrap(), &mut hls);
    assert_eq!(tmp.nextline, None);
    assert_eq!(tmp.skip_end, 19.into());
  }

  const CARD_OTMCO: [&'static str; 5] = [
    "$#         IDOTM  IDNODd  XYZUVW   IMETH  RADIUS   IELIM    ITYP   ALPHA",
    "OTMCO /        1       0  111111       0      0.                        ",
    "$#                                                                         TITLE",
    "NAME Otmco->1                                                                   ",
    "END_OTMCO",
  ];

  macro_rules! hints_index_test {
    ($name: ident, $strs: ident, $iter_from: expr, $linenr: expr, $idx: expr) => {
      #[test]
      fn $name() {
        let mut lines = Lines::new();
        lines.parse_strs(&$strs);

        let mut it = lines.iter_from($iter_from);
        let cline = it.next().unwrap().try_into_keywordline().unwrap();
        let card: &'static Card = (&cline.keyword).into();

        let cardlineidx =
          it.get_cardline_hints_index(&cline, card, LineNr::from_i64($linenr));

        assert_eq!(Some($idx), cardlineidx);
      }
    };
  }

  hints_index_test!(cardline_hints_index_first, CARD_OTMCO, 0, 1, 0);
  hints_index_test!(cardline_hints_index_second, CARD_OTMCO, 0, 3, 1);
  hints_index_test!(cardline_hints_index_block_end, CARD_OTMCO, 0, 4, 4);

}
