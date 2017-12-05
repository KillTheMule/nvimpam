//! This module is the host for some code related to nvimpam that I don't want
//! to use right now, but want to keep for later.

use cards::Card;
use std::str::Bytes;

/// [parse_str](../cards/enum.Card.html#method.parse_str) seems to largely
/// dominate the benchmark for
/// [create_card_data](../cards/enum.Card.html#method.create_card_data), this
/// might be a faster alternative. Needs real benchmarks!
#[inline]
#[allow(dead_code)]
pub fn parse_str2<'a>(s: &'a str) -> Option<Card> {
  use cards::Card::*;
  match s.bytes().next() {
    Some(b'N') if s.starts_with("NODE") => Some(Node),
    Some(b'S') if s.starts_with("SHELL") => Some(Shell),
    Some(b'$') | Some(b'#') => Some(Comment),
    _ => None,
  }
}

/// Helper function used by [parse_str3](fn.parse_str3.html).
#[inline]
#[allow(dead_code)]
pub fn check_rest(mut rest: Bytes, check: &[u8], card: Card) -> Option<Card> {
  for b in check {
    if rest.next() != Some(*b) {
      return None;
    }
  }
  Some(card)
}

/// [parse_str](../cards/enum.Card.html#method.parse_str) seems to largely
/// dominate the benchmark for
/// [create_card_data](../cards/enum.Card.html#method.create_card_data), this
/// might be a faster alternative. Needs real benchmarks!
#[inline]
#[allow(dead_code)]
pub fn parse_str3(s: &str) -> Option<Card> {
  use cards::Card::*;
  let mut bytes = s.bytes();
  match bytes.next() {
    Some(b'N') => {
      match bytes.next() {
        Some(b'O') => {
          match bytes.next() {
            Some(b'D') => check_rest(bytes, b"E", Node),
            _ => None,
          }
        }
        _ => None,
      }
    }
    Some(b'S') => check_rest(bytes, b"HELL", Shell),
    Some(b'$') | Some(b'#') => Some(Comment),
    _ => None,
  }
}

/// [parse_str](../cards/enum.Card.html#method.parse_str) seems to largely
/// dominate the benchmark for
/// [create_card_data](../cards/enum.Card.html#method.create_card_data), this
/// might be a faster alternative. Needs real benchmarks!
#[inline]
#[allow(dead_code)]
pub fn parse_str4<'a>(s: &'a str) -> Option<Card> {
  use cards::Card::*;
  use std::ptr;
  use std::cmp;

  // I only wrote the little-endian version
  assert!(cfg!(target_endian = "little"));
  const NODE: u32 = 0x45444f4e;
  const SHELL: u64 = 0x4c4c454853;

  let b = s.as_bytes();
  let mut m: u64 = 0;
  unsafe {
    ptr::copy_nonoverlapping(
      b.as_ptr(),
      &mut m as *mut u64 as *mut u8,
      cmp::min(8, b.len()),
    );
  }
  let m0 = m as u8;
  if m0 == b'$' || m0 == b'#' {
    return Some(Comment);
  }
  if m as u32 == NODE {
    return Some(Node);
  }
  if m & 0xffffffffff == SHELL {
    return Some(Shell);
  }
  None
}

/// Structure to hold fold data
#[allow(dead_code)]
pub struct Fold {
  start: u64,
  end: u64,
  card: Option<Card>,
}

/// Structure holding the original iterator I (the Vec<String> in nvimpam) and
/// the state that needs saving between next() calls, ncard. This is the next
/// fold to return without touching the original iterator. That can happen if we
/// iterated "too far" while looking for the next card type after a comment, in
/// which case ncard will be Some(Comment) and the iterator will continue after
/// the comment.
///
/// TODO: In the case above, doesn't the iterator continue on the 2nd line after
/// the comment? Don't we need to save up something here?
pub struct Folds<I> {
  orig: I,
  ncard: Option<Fold>, // saved up the next fold
}

/// Trait that creates an iterator adaptor to contract folding data [like
/// this](../cards/enum.Card.html#method.create_card_data).
pub trait FoldExt: Sized {
  fn folds(self) -> Folds<Self>;
}

/// Impl the iterator adaptors for iterators that return a numbering of card
/// classifications, as we get from Vec<String> by first mapping
/// [parse_str](../cards/enum.Card.html#method.parse_str) and then calling
/// enumerate()
impl<I> FoldExt for I
where
  I: ExactSizeIterator<Item = (usize, Option<Card>)>,
{
  fn folds(self) -> Folds<Self> {
    Folds {
      orig: self,
      ncard: None,
    }
  }
}

/// Iterating over fold data and returning a fold range for each next() call.
/// Might not be correct, see this [comment](struct.Folds.html)
impl<I> Iterator for Folds<I>
where
  I: ExactSizeIterator<Item = (usize, Option<Card>)>,
{
  type Item = Fold;

  fn next(&mut self) -> Option<Self::Item> {
    if let Some(f) = self.ncard.take() {
      return Some(f);
    }

    let len = self.orig.len();

    let mut curcardstart = 0;
    let mut curcard: Option<Card> = None;

    let mut last_before_comment = 0;

    for (ref i, ref linecard) in self.orig.by_ref() {
      match *linecard {
        None => {
          if *i > 0 {
            if last_before_comment > 0 {
              if i - last_before_comment > 1 {
                self.ncard = Some(Fold {
                  start: last_before_comment as u64 + 1,
                  end: *i as u64 - 1,
                  card: Some(Card::Comment),
                });
                return Some(Fold {
                  start: curcardstart as u64,
                  end: last_before_comment as u64,
                  card: curcard,
                });
              } else {
                return Some(Fold {
                  start: curcardstart as u64,
                  end: *i as u64 - 1,
                  card: curcard,
                });
              }
            }
          }
          curcard = None;
          curcardstart = *i;
        }
        Some(ref c) => {
          if *linecard == curcard {
            last_before_comment = 0;
            continue;
          } else {
            if *linecard == Some(Card::Comment) {
              if *i > 1 && last_before_comment == 0 {
                last_before_comment = i - 1;
                continue;
              } else {
                if *i == 0 {
                  curcard = Some(Card::Comment);
                  curcardstart = 0;
                }
              }
            } else {
              // linecard != curcard, and linecard != Some(Comment)
              if last_before_comment > 0 {
                return Some(Fold {
                  start: curcardstart as u64,
                  end: last_before_comment as u64,
                  card: curcard,
                });
              } else {
                if *i > 0 {
                  return Some(Fold {
                    start: curcardstart as u64 + 1,
                    end: *i as u64 - 1,
                    card: curcard,
                  });
                }
              }
              curcard = Some(*c);
              curcardstart = *i;
            }
          }
        }
      }
    }
    if curcardstart > 0 {
      return Some(Fold {
        start: curcardstart as u64,
        end: len as u64,
        card: curcard,
      });
    }
    None
  }
}

/// Use [FoldExt](trait.FoldExt.html) to creat the card data. Seems slower than
/// the direct way.  Might not be correct, see this [comment](struct.Folds.html)
#[inline]
#[allow(dead_code)]
pub fn create_card_data5<T: AsRef<str>>(
  lines: &[T],
) -> Vec<(Option<Card>, u64, u64)> {

  let mut v = Vec::new();
  let it = lines
    .iter()
    .map(|s| parse_str3(s.as_ref()))
    .enumerate()
    .folds();

  for fold in it {
    v.push((fold.card, fold.start, fold.end));
  }
  v
}
