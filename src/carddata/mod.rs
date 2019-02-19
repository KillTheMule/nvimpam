//! This module contains all the global static [`Card`](::card::Card) instances
//
// Export the macros inside this crate
#[cfg(test)]
macro_rules! cardtest {
  ($name: ident, $c: expr, $v: expr) => {
    #[test]
    fn $name() {
      use crate::bufdata::BufData;
      use neovim_lib::{Value, neovim_api::Buffer};

      let buf = Buffer::new(Value::from(0_usize));
      let mut bufdata = BufData::new(&buf);
      bufdata.parse_strs(&$c).unwrap();

      assert_eq!($v, bufdata.folds_to_vec());
    }
  };
  ($name: ident, $c: ident, $v: expr, $w: expr) => {
    #[test]
    fn $name() {
      use crate::bufdata::BufData;
      use neovim_lib::{Value, neovim_api::Buffer};

      let buf = Buffer::new(Value::from(0_usize));
      let mut bufdata = BufData::new(&buf);
      bufdata.parse_strs(&$c).unwrap();

      assert_eq!($v, bufdata.folds_to_vec());
      assert_eq!($w, bufdata.folds_level2_to_vec());
    }
  };
}

pub(crate) mod auxiliaries;
pub(crate) mod constraint;
pub(crate) mod element;
pub(crate) mod link;
pub(crate) mod node;
pub(crate) mod part;

/// All static declarations can be imported via
/// ```rust, compile_fail
/// use carddata::*;
/// ```
pub(crate) use self::element::*;
pub(crate) use self::{auxiliaries::*, constraint::*, link::*, node::*, part::*};
