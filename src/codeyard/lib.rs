
#[inline]
pub fn parse_str2<'a>(s: &'a str) -> Option<Card> {
  use self::Card::*;
  match s.bytes().next() {
    Some(b'N') if s.starts_with("NODE") => Some(Node),
    Some(b'S') if s.starts_with("SHELL") => Some(Shell),
    Some(b'$') | Some(b'#') => Some(Comment),
    _ => None,
  }
}
#[inline]
fn check_rest(mut rest: Bytes, check: &[u8], card: Card) -> Option<Card> {
  for b in check {
    if rest.next() != Some(*b) {
      return None;
    }
  }
  Some(card)
}
#[inline]
pub fn parse_str3(s: &str) -> Option<Card> {
  use self::Card::*;
  let mut bytes = s.bytes();
  match bytes.next() {
    Some(b'N') => {
      match bytes.next() {
        Some(b'O') => {
          match bytes.next() {
            Some(b'D') => Card::check_rest(bytes, b"E", Node),
            _ => None,
          }
        }
        _ => None,
      }
    }
    Some(b'S') => Card::check_rest(bytes, b"HELL", Shell),
    Some(b'$') | Some(b'#') => Some(Comment),
    _ => None,
  }
}
#[inline]
pub fn parse_str4<'a>(s: &'a str) -> Option<Card> {
  use self::Card::*;
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

pub struct Fold {
  start: u64,
  end: u64,
  card: Option<Card>
}

pub struct Folds<I> {
  orig: I,
  ncard: Option<Fold>, // saved up the next fold
}

pub trait FoldExt: Sized {
  fn folds(self) -> Folds<Self>;
}

impl<I> FoldExt for I
  where I: ExactSizeIterator<Item=(usize, Option<Card>)>  {
    fn folds(self) -> Folds<Self> {
      Folds { orig: self, ncard: None }
    }
  }

impl<I> Iterator for Folds<I> where I: ExactSizeIterator<Item=(usize, Option<Card>)> {
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
                self.ncard = Some(Fold{
                  start: last_before_comment as u64 + 1,
                  end: *i as u64 - 1,
                  card: Some(Card::Comment),
                });
                return Some(Fold { 
                    start: curcardstart as u64,
                    end: last_before_comment as u64,
                    card: curcard
                   });
               } else {
                return Some(Fold { 
                    start: curcardstart as u64,
                    end: *i as u64 - 1,
                    card: curcard
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
                return Some (Fold {
                  start: curcardstart as u64,
                  end: last_before_comment as u64,
                  card: curcard
                });
              } else {
                if *i > 0 {
                  return Some(Fold {
                    start: curcardstart as u64 + 1,
                    end: *i as u64 - 1,
                    card: curcard
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
        card: curcard
      });
    }
   None 
  }
}
