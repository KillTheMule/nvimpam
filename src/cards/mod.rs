//! This module provides the [Card](cards/enum.Card.html) enum to classify lines
//! according to what card type they belong to. The term "Card" is from the
//! FEM solver Pamcrash, but generally used among FEM solvers.

/// An enum to denote the several types of cards a line might belong to. For now
/// carries only information equivalent to the keyword, not the subtypes, e.g.
/// CNTAC types 33 and 36 will both be denoted by type Cntac
use std::io::Error;

use folds::FoldList;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Card {
  Node,
  Shell,
  Comment,
}

impl Card {
  /// Parse a string to determine if it starts with the keyword of a card.
  #[inline]
  pub fn parse_str<'a>(s: &'a str) -> Option<Card> {
    use self::Card::*;
    if s.starts_with("NODE") {
      return Some(Node);
    } else if s.starts_with("SHELL") {
      return Some(Shell);
    } else if s.starts_with("$") || s.starts_with("#") {
      return Some(Comment);
    } else {
      return None;
    };
  }

  /// Parse an array of strings into a
  /// [FoldList](../folds/struct.FoldList.html). The foldlist is cleared as a
  /// first step.
  ///
  /// Comments are subsumed into a fold of a different type, if the
  /// surrounding folds are of the same type. This will create a larger fold
  /// containing the surrounding folds and the comments, and will be of the
  /// type of the surrounding folds. Otherwise, folds will form their own
  /// fold range.
  #[inline]
  pub fn create_card_data<T: AsRef<str>>(
    lines: &[T],
    foldlist: &mut FoldList,
  ) -> Result<(), Box<Error>> {
    let it = lines
      .iter()
      .map(|s| Card::parse_str(s.as_ref()))
      .enumerate();
    let mut curcardstart = 0;
    let mut curcard: Option<Card> = None;
    foldlist.clear();

    let mut last_before_comment = 0;

    for (i, linecard) in it {
      match linecard {
        None => {
          if i > 0 {
            if let Some(c) = curcard {
              if last_before_comment > 0 {
                foldlist.insert(
                  curcardstart as u64,
                  last_before_comment as u64,
                  c,
                )?;
                if i - last_before_comment > 1 {
                  foldlist.insert(
                    last_before_comment as u64 + 1,
                    i as u64 - 1,
                    Card::Comment,
                  )?;
                }
                last_before_comment = 0;
              } else {
                foldlist.insert(curcardstart as u64, i as u64 - 1, c)?;
              }
            }
          }
          curcard = None;
          curcardstart = i;
        }
        Some(ref c) => {
          if linecard == curcard {
            last_before_comment = 0;
            continue;
          } else {
            if linecard == Some(Card::Comment) {
              if i > 1 && last_before_comment == 0 {
                last_before_comment = i - 1;
                continue;
              } else {
                if i == 0 {
                  curcard = Some(Card::Comment);
                  curcardstart = 0;
                }
              }
            } else {
              // linecard != curcard, and linecard != Some(Comment)
              if let Some(c) = curcard {
                if last_before_comment > 0 {
                  foldlist.insert(
                    curcardstart as u64,
                    last_before_comment as u64,
                    c,
                  )?;
                  // probably redundant
                  if i > 0 {
                    foldlist.insert(
                      last_before_comment as u64 + 1,
                      i as u64 - 1,
                      Card::Comment,
                    )?;
                  }
                  last_before_comment = 0;
                } else {
                  if i > 0 {
                    foldlist.insert(curcardstart as u64, i as u64 - 1, c)?;
                  }
                }
              }
              curcard = Some(*c);
              curcardstart = i;
            }

          }
        }
      }
    }
    // When through the whole vec, need to insert a last card
    if let Some(c) = curcard {
      foldlist.insert(
        curcardstart as u64,
        lines.len() as u64 - 1,
        c,
      )?;
    }
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use cards::Card;

  const LINES: [&'static str; 20] = [
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

  // #[test]
  // fn parse_strings() {

  #[test]
  fn fold_end_01() {
    use self::Card::*;
    use folds::FoldList;

    let mut v = vec![
      (0, 3, Node),
      (4, 4, Comment),
      (5, 5, Shell),
      (7, 15, Shell),
      (16, 17, Comment),
      (18, 19, Node),
    ];
    let mut foldlist = FoldList::new();
    let _ = Card::create_card_data(&LINES, &mut foldlist);
    assert_eq!(v, foldlist.into_vec());

    v = vec![
      (0, 0, Comment),
      (1, 1, Shell),
      (3, 11, Shell),
      (12, 13, Comment),
      (14, 15, Node),
    ];
    let mut foldlist = FoldList::new();
    let _ = Card::create_card_data(&LINES[4..], &mut foldlist);
    assert_eq!(v, foldlist.into_vec());

    v = vec![(1, 9, Shell), (10, 11, Comment), (12, 13, Node)];
    let mut foldlist = FoldList::new();
    let _ = Card::create_card_data(&LINES[6..], &mut foldlist);
    assert_eq!(v, foldlist.into_vec());
  }
}
