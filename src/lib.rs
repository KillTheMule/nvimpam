//! The companion library to the nvimpam binary.

#![feature(rust_2018_preview)]

extern crate failure;
#[macro_use]
extern crate log;
extern crate neovim_lib;
extern crate itertools;

pub mod card;
pub mod carddata;
pub mod codeyard;
pub mod event;
pub mod folds;
pub mod handler;
pub mod lines;
pub mod nocommentiter;
pub mod skipresult;
