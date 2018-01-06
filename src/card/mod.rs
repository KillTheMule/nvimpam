pub mod keyword;
pub mod line;
pub mod cell;
pub mod ges;

use std::iter;
use std::slice;

use self::line::Line;
use self::keyword::Keyword;
use carddata::*;

#[derive(Debug)]
pub struct Card {
  pub lines: &'static [Line],
  pub ownfold: bool,
  pub keyword: Keyword,
}

impl<'a> From<&'a Keyword> for &'static Card {
  fn from(kw: &'a Keyword) -> &'static Card {
    match kw {
      &Keyword::Node => &NODE,
      &Keyword::Cnode => &CNODE,
      &Keyword::Shell => &SHELL,
      &Keyword::Comment => &COMMENT,
      &Keyword::Mass => &MASS,
      &Keyword::Nsmas => &NSMAS,
      &Keyword::Nsmas2 => &NSMAS2,
    }
  }
}

impl Card {
  #[inline]
  fn get_foldend_own<'a, T: AsRef<str>>(
    &self,
    it: &mut iter::Enumerate<slice::Iter<'a, T>>,
  ) -> (Option<u64>, Option<Keyword>, Option<u64>) {
    let num = self.lines.len();
    let mut i = 0;
    let mut idx = 0;
    let mut line;


    while i < num {
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
          // i += 1;
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
  }


  #[inline]
  fn get_foldend_gather<'a, T: AsRef<str>>(
    &self,
    it: &mut iter::Enumerate<slice::Iter<'a, T>>,
  ) -> (Option<u64>, Option<Keyword>, Option<u64>) {
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

    while curkw == Some(self.keyword) || curkw == Some(Keyword::Comment) {
      if curkw == Some(Keyword::Comment) && idx_before_comment == 0 {
        idx_before_comment = idx - 1;
      } else if curkw == Some(self.keyword) && idx_before_comment > 0 {
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
