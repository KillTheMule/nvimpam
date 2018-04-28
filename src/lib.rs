//! The companion library to the nvimpam binary.
extern crate failure;
#[macro_use]
extern crate log;
extern crate neovim_lib;

pub mod card;
pub mod carddata;
pub mod codeyard;
pub mod event;
pub mod folds;
pub mod handler;
pub mod lines;
pub mod neovim_ext;
