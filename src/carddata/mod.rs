//! This module contains all the global static [`Card`](crate::card::Card)
//! instances
//
// Export the macros inside this crate
#[cfg(test)]
macro_rules! cardtest {
  ($name: ident, $c: expr, $v: expr) => {
    #[test]
    fn $name() {
      use crate::bufdata::BufData;
      use neovim_lib::{neovim_api::Buffer, Value};

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
      use neovim_lib::{neovim_api::Buffer, Value};

      let buf = Buffer::new(Value::from(0_usize));
      let mut bufdata = BufData::new(&buf);
      bufdata.parse_strs(&$c).unwrap();

      assert_eq!($v, bufdata.folds_to_vec());
      assert_eq!($w, bufdata.folds_level2_to_vec());
    }
  };
}

//pub mod auxiliaries;
pub mod constraint;
//pub mod element;
//pub mod link;
pub mod node;
//pub mod part;

pub use self::constraint::*;
/// All static declarations can be imported via
/// ```rust, compile_fail
/// use carddata::*;
/// ```
//pub use self::element::*;
//pub use self::{auxiliaries::*, constraint::*, link::*, node::*,
// part::*};
pub use self::node::*;
