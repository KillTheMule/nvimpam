use std::default::Default;

use skipresult::SkipResult;
use card::ges::GesType;
use card::keyword::Keyword;
use card::line::Line;
use card::Card;

/// Designates that the comments have been removed.
pub trait CommentLess {
  fn remove_comments(self) -> NoCommentIter<Self>
  where
    Self: Sized;
}

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
      let t = n.as_ref();
      let l = t.len();
      if !(l > 0 && (t.as_bytes()[0] == b'#' || t.as_bytes()[0] == b'$')) {
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
  /// Advance the iterator until meeting the first line with a keyword. Return
  /// the index and a reference to that line. If no line starts with a
  /// keyword, return `None`.
  ///
  /// NOTE: A Comment line counts as a keyword. Also see
  /// `skip_to_next_real_keyword`.
  pub fn skip_to_next_keyword<'b>(&'b mut self) -> SkipResult<'a, T> {
    let mut prevline = None;
    let mut nextline;
    let mut done = false;

    nextline = self.next();

    if nextline.is_none() {
      return Default::default();
    }

    if Keyword::parse(nextline.unwrap().1).is_some() {
      done = true;
    }

    while !done {
      prevline = nextline;
      nextline = self.next();
      if nextline.is_none() {
        return Default::default();
      }
      if Keyword::parse(nextline.unwrap().1).is_some() {
        done = true;
      }
    }

    let last = match prevline {
      Some((i, _l)) => Some(i),
      None => None,
    };

    match nextline {
      None => SkipResult {
        nextline: None,
        nextline_kw: None,
        skip_end: None,
      },
      Some((_i, l)) => SkipResult {
        nextline,
        nextline_kw: Keyword::parse(&l),
        skip_end: last,
      },
    }
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
  pub fn skip_ges<'b>(&'b mut self, ges: &GesType) -> SkipResult<'a, T> {
    let mut previdx = None;
    let mut idx;
    let mut line;
    let mut ges_contains_line;

    let tmp = self.next();
    match tmp {
      None => return Default::default(),
      Some((i, l)) => {
        idx = i;
        line = l;
        ges_contains_line = ges.contains(&line);
      }
    }

    while ges_contains_line {
      let tmp = self.next();
      match tmp {
        None => {
          return SkipResult {
            skip_end: Some(idx),
            ..Default::default()
          };
        }
        Some((i, l)) => {
          previdx = Some(idx);
          ges_contains_line = ges.contains(&l);
          idx = i;
          line = l;
        }
      }
    }

    if ges.ended_by(&line) {
      previdx = Some(idx);
      let nextline = self.next();

      match nextline {
        None => SkipResult {
          skip_end: previdx,
          ..Default::default()
        },
        Some((i, l)) => SkipResult {
          nextline: Some((i, l)),
          nextline_kw: Keyword::parse(l),
          skip_end: previdx,
        },
      }
    } else {
      SkipResult {
        nextline: Some((idx, line)),
        nextline_kw: Keyword::parse(&line),
        skip_end: previdx,
      }
    }
  }

  pub fn skip_fold<'b>(&'b mut self, card: &Card) -> SkipResult<'a, T> {
    if card.ownfold {
      self.skip_card(card)
    } else {
      self.skip_card_gather(card)
    }
  }

  pub fn skip_card<'b>(&'b mut self, card: &Card) -> SkipResult<'a, T> {
    let mut cardlines = card.lines.iter();
    let mut conds: Vec<bool> = vec![]; // the vec to hold the conditionals

    // We've already seen this line from the iterator
    let _ = cardlines.next();

    let mut line; // line of the iterator we're currently processing
    let mut lineidx; // index of the currently processed line
    let mut linekw; // Keyword of the currently processed line
    let mut previdx = None; // index of the last line of the card
    let mut tmp;

    match self.next() {
      None => return Default::default(),
      Some((i, l)) => {
        line = l;
        lineidx = i;
        linekw = Keyword::parse(line);
      }
    }

    for cardline in cardlines {
      match *cardline {
        Line::Provides(_s, ref c) => conds.push(c.evaluate(&line)),
        Line::Ges(ref g) => {
          tmp = self.skip_ges(g);
          match tmp.nextline {
            None => match tmp.skip_end {
              None => return Default::default(),
              Some(i) => {
                return SkipResult {
                  skip_end: Some(i),
                  ..Default::default()
                }
              }
            },
            Some((i, l)) => {
              if let Some(j) = tmp.skip_end {
                line = l;
                lineidx = i;
                linekw = tmp.nextline_kw;
                previdx = Some(j);
              } else {
                unreachable!();
              }
            }
          }
        }
        Line::Cells(_s) => {
          if linekw.is_some() {
            return SkipResult {
              nextline: Some((lineidx, line)),
              nextline_kw: linekw,
              skip_end: previdx,
            };
          } else {
            let tmp = self.next();
            match tmp {
              None => {
                return SkipResult {
                  skip_end: Some(lineidx),
                  ..Default::default()
                };
              }
              Some((i, l)) => {
                previdx = Some(lineidx);
                line = l;
                lineidx = i;
                linekw = Keyword::parse(l);
              }
            }
          }
        }
        Line::Optional(_s, i) => {
          if conds.get(i as usize) != Some(&true) {
            continue;
          } else if let Some(kw) = Keyword::parse(line) {
            return SkipResult {
              nextline: Some((lineidx, line)),
              nextline_kw: Some(kw),
              skip_end: previdx,
            };
          } else {
            let tmp = self.next();
            match tmp {
              None => {
                return SkipResult {
                  skip_end: Some(lineidx),
                  ..Default::default()
                };
              }
              Some((i, l)) => {
                previdx = Some(lineidx);
                line = l;
                lineidx = i;
                linekw = Keyword::parse(l);
              }
            }
          }
        }
      }
    }

    SkipResult {
      nextline: Some((lineidx, line)),
      nextline_kw: linekw,
      skip_end: previdx,
    }
  }

  pub fn skip_card_gather<'b>(&'b mut self, card: &Card) -> SkipResult<'a, T> {
    let mut curkw;
    let mut res;
    let mut curidx;
    let mut curline;
    let mut previdx = None;

    loop {
      res = self.skip_card(card);

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
  use card::ges::GesType;
  use card::keyword::Keyword;
  use card::Card;
  use carddata::*;
  use nocommentiter::CommentLess;

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
    {
      let nextline = li.it.next();
      assert_eq!(nextline, Some((0, &v[0])));
    }
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
    let mut li = COMMENTS
      .iter()
      .skip(4)
      .enumerate()
      .remove_comments();
    assert_eq!(li.skip_to_next_keyword().nextline, None);
  }

  #[test]
  fn linesiter_finds_real_keywords() {
    let mut li = KEYWORD_LINES
      .iter()
      .enumerate()
      .remove_comments();
    {
      assert_eq!(
        li.skip_to_next_keyword().nextline,
        Some((2, &KEYWORD_LINES[2]))
      );
      assert_eq!(
        li.skip_to_next_keyword().nextline,
        Some((4, &KEYWORD_LINES[4]))
      );
      assert_eq!(li.skip_to_next_keyword().nextline, None);
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
    let g = GesType::GesNode;
    let mut li = GES1.iter().enumerate().remove_comments();

    assert_eq!(li.skip_ges(&g).nextline, Some((4, &GES1[4])));
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
    let g = GesType::GesNode;
    let mut li = GES2.iter().enumerate().remove_comments();

    assert_eq!(li.skip_ges(&g).nextline, Some((3, &GES2[3])));
    assert_eq!(li.skip_ges(&g).nextline, None);
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
    let g = GesType::GesNode;
    let mut li = GES3.iter().enumerate().remove_comments();

    assert_eq!(li.skip_ges(&g).nextline, Some((2, &GES3[2])));
    assert_eq!(li.skip_ges(&g).nextline, Some((7, &GES3[7])));
    assert_eq!(li.next(), Some((8, &GES3[8])));
  }

  const GES4: [&'static str; 2] = ["wupdiwup", "NODE  / "];

  #[test]
  fn ges_can_skip_nothing() {
    let g = GesType::GesNode;
    let mut li = GES4.iter().enumerate().remove_comments();

    assert_eq!(li.skip_ges(&g).nextline, Some((0, &GES4[0])));
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
    let g = GesType::GesNode;
    let mut li = GES6.iter().enumerate().remove_comments();

    let tmp = li.skip_ges(&g);
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
    let g = GesType::GesNode;
    let mut li = GES7.iter().enumerate().remove_comments();

    let tmp = li.skip_ges(&g);
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
    let g = GesType::GesNode;
    let mut li = GES8.iter().enumerate().remove_comments();

    let tmp = li.skip_ges(&g);
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
    let firstline = li.next();
    let kw: Keyword = Keyword::parse(&firstline.unwrap().1).unwrap();
    let k = &kw;
    let card: &'static Card = k.into();

    let tmp = li.skip_card(card);
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
    let firstline = li.next();
    let kw: Keyword = Keyword::parse(&firstline.unwrap().1).unwrap();
    let k = &kw;
    let card: &'static Card = k.into();

    let tmp = li.skip_card_gather(card);
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
    let firstline = li.next();
    let kw: Keyword = Keyword::parse(&firstline.unwrap().1).unwrap();
    let k = &kw;
    let card: &'static Card = k.into();

    let tmp = li.skip_card(card);
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
    let mut li = CARD_MASS_OPT
      .iter()
      .enumerate()
      .remove_comments();
    let firstline = li.next();
    let kw: Keyword = Keyword::parse(&firstline.unwrap().1).unwrap();
    let k = &kw;
    let card: &'static Card = k.into();

    let tmp = li.skip_card(card);
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
    "SHELL /     3129       1       1    2967    2971    2970",
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
    let mut li = LINES_GATHER
      .iter()
      .enumerate()
      .remove_comments();
    let firstline = li.it.next();
    let kw: Keyword = Keyword::parse(&firstline.unwrap().1).unwrap();
    let k = &kw;
    let card: &'static Card = k.into();

    let mut tmp = li.skip_fold(card);
    assert_eq!(tmp.nextline, Some((5, &LINES_GATHER[5])));
    assert_eq!(tmp.skip_end, Some(3));

    tmp = li.skip_fold(&SHELL);
    assert_eq!(tmp.nextline, Some((6, &LINES_GATHER[6])));
    assert_eq!(tmp.skip_end, Some(5));

    let _ = li.it.next();
    tmp = li.skip_fold(&SHELL);
    assert_eq!(tmp.nextline, Some((18, &LINES_GATHER[18])));
    assert_eq!(tmp.skip_end, Some(15));

    tmp = li.skip_fold(&NODE);
    assert_eq!(tmp.nextline, None);
    assert_eq!(tmp.skip_end, None);
  }

}
