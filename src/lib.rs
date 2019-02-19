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
pub(crate) mod carddata;
pub mod bufdata;
pub(crate) mod card;
pub mod event;
pub mod handler;
pub(crate) mod linenr;
pub(crate) mod lines;
pub(crate) mod linesiter;
pub(crate) mod skipresult;
