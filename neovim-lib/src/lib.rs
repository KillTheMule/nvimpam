//! # Rust library for Neovim clients
//!
//! Implements support for rust plugins for [Neovim](https://github.com/neovim/neovim) through its msgpack-rpc API.
//! # Examples
//! ```no_run
//! use neovim_lib::{Neovim, NeovimApi, Session};
//!
//! let mut session = Session::new_tcp("127.0.0.1:6666").unwrap();
//! session.start_event_loop();
//! let mut nvim = Neovim::new(session);
//!
//! let buffers = nvim.list_bufs().unwrap();
//! buffers[0].set_lines(&mut nvim, 0, 0, true, vec!["replace first line".to_owned()]).unwrap();
//! nvim.command("vsplit").unwrap();
//! let windows = nvim.list_wins().unwrap();
//! windows[0].set_width(&mut nvim, 10).unwrap();
//! ```
extern crate rmp;
extern crate rmpv;
#[macro_use]
extern crate log;

#[cfg(unix)]
extern crate unix_socket;

mod rpc;
#[macro_use]
pub mod session;
pub mod neovim;
pub mod neovim_api;

pub use neovim::{Neovim, UiAttachOptions, UiOption, CallError};
pub use neovim_api::NeovimApi;
pub use session::Session;

pub use rpc::handler::{Handler};
pub use rmpv::{Value, Integer};
