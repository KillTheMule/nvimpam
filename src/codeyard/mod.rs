//! This module is the host for some code related to nvimpam that I don't want
//! to use right now, but want to keep for later.
use std::str::Bytes;

use failure::Error;

use card::keyword::Keyword;
use card::Card;
use folds::FoldList;


/// [parse_str](../cards/enum.Keyword.html#method.parse_str) seems to largely
/// dominate the benchmark for
/// [create_card_data](../cards/enum.Keyword.html#method.create_card_data), this
/// might be a faster alternative. Needs real benchmarks!
#[inline]
#[allow(dead_code)]
pub fn parse_str2<'a>(s: &'a str) -> Option<Keyword> {
  use card::keyword::Keyword::*;
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
pub fn check_rest(
  mut rest: Bytes,
  check: &[u8],
  card: Keyword,
) -> Option<Keyword> {
  for b in check {
    if rest.next() != Some(*b) {
      return None;
    }
  }
  Some(card)
}

/// [parse_str](../cards/enum.Keyword.html#method.parse_str) seems to largely
/// dominate the benchmark for
/// [create_card_data](../cards/enum.Keyword.html#method.create_card_data), this
/// might be a faster alternative. Needs real benchmarks!
#[inline]
#[allow(dead_code)]
pub fn parse_str3(s: &str) -> Option<Keyword> {
  use card::keyword::Keyword::*;
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

/// [parse_str](../cards/enum.Keyword.html#method.parse_str) seems to largely
/// dominate the benchmark for
/// [create_card_data](../cards/enum.Keyword.html#method.create_card_data), this
/// might be a faster alternative. Needs real benchmarks!
#[inline]
#[allow(dead_code)]
pub fn parse_str4<'a>(s: &'a str) -> Option<Keyword> {
  use card::keyword::Keyword::*;
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
  card: Option<Keyword>,
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
/// this](../cards/enum.Keyword.html#method.create_card_data).
pub trait FoldExt: Sized {
  fn folds(self) -> Folds<Self>;
}

/// Impl the iterator adaptors for iterators that return a numbering of card
/// classifications, as we get from Vec<String> by first mapping
/// [parse_str](../cards/enum.Keyword.html#method.parse_str) and then calling
/// enumerate()
impl<I> FoldExt for I
where
  I: ExactSizeIterator<Item = (usize, Option<Keyword>)>,
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
  I: ExactSizeIterator<Item = (usize, Option<Keyword>)>,
{
  type Item = Fold;

  fn next(&mut self) -> Option<Self::Item> {
    if let Some(f) = self.ncard.take() {
      return Some(f);
    }

    let len = self.orig.len();

    let mut curcardstart = 0;
    let mut curcard: Option<Keyword> = None;

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
                  card: Some(Keyword::Comment),
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
            if *linecard == Some(Keyword::Comment) {
              if *i > 1 && last_before_comment == 0 {
                last_before_comment = i - 1;
                continue;
              } else {
                if *i == 0 {
                  curcard = Some(Keyword::Comment);
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
) -> Vec<(Option<Keyword>, u64, u64)> {

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

/// Alternative to add_folds on a foldlist
/// DOES NOT WORK. Needs to properly deal with the results of get_foldend2
#[allow(dead_code)]
pub fn add_folds2<T: AsRef<str>>(
  foldlist: &mut FoldList,
  lines: &[T],
) -> Result<(), Error> {

  if lines.len() == 0 {
    return Err(format_err!("No lines passed!"));
  }
  // Iterate over the lines once
  let mut lines_enumerated_without_comments =
    Box::new(lines.iter().enumerate().filter(|&(_, l)| {
      Keyword::parse(l.as_ref()) != Some(Keyword::Comment)
    }));
  // Iterator may be advanced by this loop or `get_foldend`
  while let Some((cur_idx, cur_line)) =
    lines_enumerated_without_comments.next()
  {
    let mut cur_kw;
    match Keyword::parse(cur_line.as_ref()) {
      None => continue,
      // None |
      // Some(Keyword::Comment) => continue,
      Some(kw) => cur_kw = kw, 
    }

    match cur_kw {
      Keyword::Comment => unreachable!(),
      c => {
        let foldend = get_foldend2(&c, &mut lines_enumerated_without_comments);
        if let Some(i) = foldend.0 {
          foldlist.checked_insert(cur_idx as u64, i, c)?;
        }
        //cur_kw = foldend.1.unwrap();

        //if let Some(i) = foldend.2 {
        //  cur_idx = i as usize;
        //}
      }
    }
  }
  return Ok(());
}

/// Alternative to get_foldend to use with add_folds2
/// Needs a trait object because the use of filter make the types not writeable
#[inline]
pub fn get_foldend2<'a, T: AsRef<str>>(
  kw: &Keyword,
  it: &mut Iterator<Item = (usize, T)>,
) -> (Option<u64>, Option<Keyword>, Option<u64>) {
  let card: &Card = kw.into();

  if card.ownfold {
    let num = card.lines.len();
    let mut i = 0;
    let mut idx = 0;
    let mut line;


    while i < num {
      println!("{}", i);
      let tmp = it.next();
      match tmp {
        None => return (None, None, None),
        Some((j, l)) => {
          idx = j;
          line = l;
        }
      }

      if let Some(k) = Keyword::parse(line) {
        if k == Keyword::Comment {
          i += 1;
          continue;
        }
        return (None, Some(k), Some(idx as u64));
      } else {
        i += 1;
      }
    }

    let tmp = it.next();
    match tmp {
      None => return (Some(idx as u64), None, None),
      Some((i, l)) => {
        return (Some(idx as u64), Keyword::parse(l), Some(i as u64))
      }
    }
  } else {
    // !card.ownfold
    let mut idx;
    let mut line;
    let mut curkw;
    let mut idx_before_comment = 0;

    let tmp = it.next();
    match tmp {
      None => return (None, None, None),
      Some((j, l)) => {
        idx = j;
        line = l;
        curkw = Keyword::parse(line);
      }
    }

    if curkw.is_none() {
      return (None, None, None);
    }

    while curkw == Some(*kw) || curkw == Some(Keyword::Comment) {
      if curkw == Some(Keyword::Comment) && idx_before_comment == 0 {
        idx_before_comment = idx - 1;
      } else if curkw == Some(*kw) && idx_before_comment > 0 {
        idx_before_comment = 0;
      }
      let tmp = it.next();
      match tmp {
        None => return (Some(idx as u64), None, None),
        Some((j, l)) => {
          idx = j;
          line = l;
          curkw = Keyword::parse(line);
        }
      }
    }

    if idx_before_comment > 0 {
      return (Some(idx_before_comment as u64), curkw, Some(idx as u64));
    } else {
      return (Some(idx as u64 - 1), curkw, Some(idx as u64));
    }
  }
}
