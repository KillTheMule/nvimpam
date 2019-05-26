#![deny(bare_trait_objects)]
//! The companion library to the nvimpam binary.

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
