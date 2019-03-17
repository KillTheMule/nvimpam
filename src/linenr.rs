//! The struct for linenumbers.
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]
use std::{
  fmt, i32, isize,
  ops::{Add, AddAssign, Sub},
  u32,
};

use neovim_lib::Value;

/// Wraps a `u32`, but we actually make sure on construction to stay in
/// `i32` range. This *should* hold through all operations, but we actually
/// only check this in debug mode. It will become a problem if you're dealing
/// with more than `i32::max_value()` lines, which is 2G.
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub struct LineNr(u32);

impl LineNr {
  pub fn from_i64(num: i64) -> Self {
    debug_assert!(num >= 0);
    debug_assert!(num <= i64::from(i32::max_value()));

    Self(num as u32)
  }

  pub fn from_usize(num: usize) -> Self {
    debug_assert!(
      num <= u32::max_value() as usize,
      "Got a usize > u32::max_value()!"
    );
    Self(num as u32)
  }

  pub fn from_isize(num: isize) -> Self {
    debug_assert!(num >= 0, "Negative isizes can't be LineNrs!");
    debug_assert!(num <= i32::max_value() as isize);
    Self(num as u32)
  }

  pub fn prev(self) -> Self {
    debug_assert!(self.0 >= 1, "LineNr 0 has no previous LineNr!");
    Self(self.0 - 1)
  }
}

impl From<LineNr> for usize {
  fn from(l: LineNr) -> Self {
    l.0 as Self
  }
}

impl From<usize> for LineNr {
  fn from(num: usize) -> Self {
    Self::from_usize(num)
  }
}

impl Add for LineNr {
  type Output = Self;

  fn add(self, other: Self) -> Self::Output {
    let res = self.0 + other.0;
    debug_assert!(res <= i32::max_value() as u32);
    Self(res)
  }
}

impl Add<isize> for LineNr {
  type Output = Self;

  fn add(self, other: isize) -> Self::Output {
    // Cast is lossless, see the comment for [`LineNr`](::linenr::LineNr)
    let res = self.0 as isize + other;
    debug_assert!(res >= 0 && res <= i32::max_value() as isize);
    Self(res as u32)
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
    let res = self.0 as isize + other;
    // Casts are lossless, see the comment for [`LineNr`](::linenr::LineNr)
    debug_assert!(res >= 0 && res <= i32::max_value() as isize);
    self.0 = res as u32;
  }
}

impl AddAssign<LineNr> for LineNr {
  fn add_assign(&mut self, other: Self) {
    self.0 += other.0;
    debug_assert!(self.0 <= i32::max_value() as u32);
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
