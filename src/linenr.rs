//! The struct for linenumbers.
use std::{
  fmt, isize, mem,
  ops::{Add, AddAssign, Sub},
};

use neovim_lib::Value;

/// Wraps a `usize`, but we actually make sure on construction to stay in
/// `isize` range. This *should* hold through all operations, but we actually
/// only check this in debug mode. It will become a problem if you're dealing
/// with more than `isize::MAX` lines, which is only 32k on 16bit, which we
/// don't support really, and 2G on 32bit. I'm not saying I'll be dead when this
/// becomes a problem, but see if you can make me care about 32bit by then :)
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub struct LineNr(usize);

impl LineNr {
  pub fn from_i64(num: i64) -> Self {
    debug_assert!(num >= 0);

    // We trust the compiler to optimize that out
    match mem::size_of::<usize>() {
      8 => Self(num as usize),
      4 => {
        assert!(num <= (isize::MAX as i64), "Got a i64 > isize::MAX!");
        Self(num as usize)
      }
      _ => unimplemented!(
        "Can only compile on architectures with an usize of 64bit or 32bit!"
      ),
    }
  }

  pub fn from_usize(num: usize) -> Self {
    debug_assert!(num <= isize::MAX as usize, "Got a usize > isize::MAX!");
    LineNr(num)
  }

  pub fn from_isize(num: isize) -> Self {
    debug_assert!(num >= 0, "Negative isizes can't be LineNrs!");
    LineNr(num as usize)
  }

  pub fn prev(&self) -> Self {
    debug_assert!(self.0 >= 1, "LineNr 0 has no previous LineNr!");
    LineNr(self.0 - 1)
  }
}

impl From<LineNr> for usize {
  fn from(l: LineNr) -> Self {
    l.0
  }
}

impl From<usize> for LineNr {
  fn from(num: usize) -> Self {
    LineNr(num)
  }
}

impl Add for LineNr {
  type Output = LineNr;

  fn add(self, other: Self) -> Self::Output {
    let res = self.0 + other.0;
    debug_assert!(res <= isize::MAX as usize);
    Self(res)
  }
}

impl Add<isize> for LineNr {
  type Output = LineNr;

  fn add(self, other: isize) -> Self::Output {
    // Cast is lossless, see the comment for [`LineNr`](::linenr::LineNr)
    let res = self.0 as isize + other;
    debug_assert!(res >= 0 && res <= isize::MAX);
    Self(res as usize)
  }
}

impl Sub for LineNr {
  type Output = isize;

  fn sub(self, other: Self) -> Self::Output {
    // Casts are lossless, see the comment for [`LineNr`](::linenr::LineNr)
    self.0 as isize - other.0 as isize
  }
}

impl AddAssign<isize> for LineNr {
  fn add_assign(&mut self, other: isize) {
    // Casts are lossless, see the comment for [`LineNr`](::linenr::LineNr)
    debug_assert!(
      self.0 as isize + other >= 0 && self.0 as isize + other <= isize::MAX
    );
    self.0 = Self::from_isize(self.0 as isize + other).0;
  }
}

impl fmt::Display for LineNr {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl From<LineNr> for Value {
  fn from(v: LineNr) -> Self {
    Value::Integer(From::from(v.0))
  }
}
