#![cfg_attr(feature="cargo-clippy", feature(tool_lints))]
//! The companion library to the nvimpam binary.
extern crate failure;
#[macro_use]
extern crate log;
extern crate itertools;
extern crate neovim_lib;
extern crate atoi;
extern crate byteorder;

pub mod card;
pub mod carddata;
pub mod event;
pub mod bufdata;
pub mod handler;
pub mod lines;
pub mod nocommentiter;
pub mod skipresult;
