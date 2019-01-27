//! This module contains all the global static [`Card`](::card::Card) instances
//
// Export the macros inside this crate
#[cfg(test)]
macro_rules! cardtest {
  ($name: ident, $c: expr, $v: expr) => {
    #[test]
    fn $name() {
      use crate::bufdata::BufData;

      let mut bufdata = BufData::new();
      bufdata.from_strs(&$c);

      assert_eq!($v, bufdata.folds.to_vec());
    }
  };
  ($name: ident, $c: ident, $v: expr, $w: expr) => {
    #[test]
    fn $name() {
      use crate::bufdata::BufData;

      let mut bufdata = BufData::new();
      bufdata.from_strs(&$c);

      assert_eq!($v, bufdata.folds.to_vec());
      assert_eq!($w, bufdata.folds_level2.to_vec());
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
