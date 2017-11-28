//! This module provides the [Card](enum.Card.html) enum to classify lines
//! according to what card type they belong to. The term "Card" is from the
//! FEM solver Pamcrash, but generally used among FEM solvers.

/// An enum to denote the several types of cards a line might belong to. For now
/// carries only information equivalent to the keyword, not the subtypes, e.g.
/// CNTAC types 33 and 36 will both be denoted by type Cntac
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Card {
  Node,
  Shell,
  Comment,
}

impl Card {
  // Parse a string to determine if it starts with the keyword of a card.
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

  // Parse a list of strings into a Vec of the same length containing the card
  // types of the lines
  pub fn create_card_data<T: AsRef<str>>(lines: &[T]) -> Vec<Option<Card>> {
    let mut v = Vec::with_capacity(lines.len());
    v.extend(lines.iter().map(|s| Card::parse_str(s.as_ref())));
    v
  }

  pub fn contract_card_data(
    cd: &Vec<Option<Card>>,
  ) -> Vec<(Option<Card>, u64, u64)> {

    let mut v = vec![];

    let it = cd.iter().enumerate();
    let mut curcardstart = 0;
    let mut curcard: Option<Card> = None;

    let mut last_before_comment = 0;

    for (i, linecard) in it {
      match linecard {
        &None => {
          if i > 0 {
            if last_before_comment > 0 {
              v.push((curcard, curcardstart as u64, last_before_comment as u64));
              if i - last_before_comment > 1 {
                v.push((Some(Card::Comment), last_before_comment as u64 + 1, i as u64
                     -1));
              }
              last_before_comment = 0;
             } else {
              v.push((curcard, curcardstart as u64, i as u64 - 1));
             }
          }
          curcard = None;
          curcardstart = i;
        }
        &Some(ref c) => {
          if *linecard == curcard {
            last_before_comment = 0;
            continue;
          } else {
            if *linecard == Some(Card::Comment) {
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
              if last_before_comment > 0 {
                v.push(
                  (curcard, curcardstart as u64, last_before_comment as u64),
                );
                // probably redundant
                if i > 0 {
                  v.push((
                    Some(Card::Comment),
                    last_before_comment as u64 + 1,
                    i as u64 - 1,
                  ));
                }
                last_before_comment = 0;
              } else {
                if i > 0 {
                  v.push((curcard, curcardstart as u64, i as u64 - 1));
                }
              }
              curcard = Some(*c);
              curcardstart = i;
            }

          }
        }
      }
    }
    if curcardstart > 0 {
      v.push((curcard, curcardstart as u64, cd.len() as u64 - 1));
    }
    v
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

  #[test]
  fn fold_end_01() {
    use self::Card::*;

    let mut cd = Card::create_card_data(&LINES);
    assert_eq!(cd[0], Some(Node));
    assert_eq!(cd[6], None);
    assert_eq!(cd[15], Some(Shell));
    assert_eq!(cd[16], Some(Comment));
    assert_eq!(cd[17], Some(Comment));

    let mut compacted = Card::contract_card_data(&cd);
    let mut v = vec![
      (Some(Node), 0, 3),
      (Some(Comment), 4, 4),
      (Some(Shell), 5, 5),
      (None, 6, 6),
      (Some(Shell), 7, 15),
      (Some(Comment), 16, 17),
      (Some(Node), 18, 19),
    ];
    assert_eq!(v, compacted);

    cd = Card::create_card_data(&LINES[4..]);
    compacted = Card::contract_card_data(&cd);
    v = vec![
      (Some(Comment), 0, 0),
      (Some(Shell), 1, 1),
      (None, 2, 2),
      (Some(Shell), 3, 11),
      (Some(Comment), 12, 13),
      (Some(Node), 14, 15),
    ];
    assert_eq!(v, compacted);


    cd = Card::create_card_data(&LINES[6..]);
    compacted = Card::contract_card_data(&cd);
    v = vec![
      (None, 0, 0),
      (Some(Shell), 1, 9),
      (Some(Comment), 10, 11),
      (Some(Node), 12, 13),
    ];
    assert_eq!(v, compacted);
  }
}
