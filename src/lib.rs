#![cfg_attr(feature = "cargo-clippy", feature(tool_lints))]
//! The companion library to the nvimpam binary.
extern crate failure;
#[macro_use]
extern crate log;
extern crate atoi;
extern crate byteorder;
extern crate itertools;
extern crate neovim_lib;

#[macro_use]
pub mod carddata;
pub mod bufdata;
pub mod card;
pub mod event;
pub mod handler;
pub mod linenr;
pub mod lines;
pub mod linesiter;
pub mod skipresult;
