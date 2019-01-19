//! This module contains all the global static [`Card`](::card::Card) instances
//
// Export the macros inside this crate
#![macro_use]

#[cfg(test)]
macro_rules! cardtest {
  ($name: ident, $c: expr, $v: expr) => {
    #[test]
    fn $name() {
      use crate::card::keyword::Keyword;
      use crate::bufdata::folds::FoldList;
      use crate::lines::Lines;

      let lines = Lines::from_strs(&$c);
      let keywords: Vec<_> =
        lines.iter().map(|l| Keyword::parse(l.as_ref())).collect();

      let mut foldlist = FoldList::new();
      let _ = foldlist.recreate_all(&keywords, &lines);

      assert_eq!($v, foldlist.to_vec(1));
    }
  };
  ($name: ident, $c: ident, $v: expr, $w: expr) => {
    #[test]
    fn $name() {
      use crate::card::keyword::Keyword;
      use crate::bufdata::folds::FoldList;
      use crate::lines::Lines;

      let lines = Lines::from_strs(&$c);
      let keywords: Vec<_> =
        lines.iter().map(|l| Keyword::parse(l.as_ref())).collect();

      let mut foldlist = FoldList::new();
      let _ = foldlist.recreate_all(&keywords, &lines);

      assert_eq!($v, foldlist.to_vec(1));
      assert_eq!($w, foldlist.to_vec(2));
    }
  };
}

pub mod auxiliaries;
pub mod constraint;
pub mod element;
pub mod link;
pub mod node;
pub mod part;

/// All static declarations can be imported via
/// ```rust, compile_fail
/// use carddata::*;
/// ```
pub use self::element::*;
pub use self::{auxiliaries::*, constraint::*, link::*, node::*, part::*};
