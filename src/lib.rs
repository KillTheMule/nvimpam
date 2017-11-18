//! The companion library to the nvimpam binary.
#[macro_use]
extern crate log;
extern crate neovim_lib;
extern crate simplelog;

pub mod handler;
pub mod event;
pub mod folds;
pub mod neovim_ext;
