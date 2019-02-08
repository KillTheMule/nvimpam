//! Pretty minimal example of a binary utilizing nvimpam_lib. Needs to run from
//! the project directory so neovim can find the runtime files.
//!
//! Uses the sytem allocator so it can be run under valgrind properly. Does not
//! utilize the [event loop](::event::Event:: event_loop), but makes its own so
//! it can exit after sending the first batch of folds.
extern crate neovim_lib;
extern crate nvimpam_lib;

use std::alloc::System;
#[global_allocator]
static GLOBAL: System = System;

use std::{path::Path, process::Command, sync::mpsc, fs};

use nvimpam_lib::{
  bufdata::BufData, event::Event::*, handler::NeovimHandler
};

use neovim_lib::{neovim::Neovim, neovim_api::NeovimApi, session::Session};

fn main() {
  let (handler_to_main, main_from_handler) = mpsc::channel();
  let (_main_to_handler, handler_from_main) = mpsc::channel();
  let nvimpath = Path::new("neovim").join("build").join("bin").join("nvim");

  let mut session = Session::new_child_cmd(
    Command::new(nvimpath)
      .args(&["-u", "NONE", "--embed"])
      .env("VIMRUNTIME", "neovim/runtime"),
  )
  .unwrap();

  session.start_event_loop_handler(NeovimHandler {
    to_main: handler_to_main,
    from_main: handler_from_main,
  });
  let mut nvim = Neovim::new(session);

  nvim.command("set noswapfile").expect("0");
  nvim.command("execute 'set rtp +='.getcwd()").expect("1");
  nvim.command("silent e! files/example.pc").expect("2");
  let curbuf = nvim.get_current_buf().expect("3");

  let origlines = fs::read("files/example.pc").expect("3.1");
  let mut bufdata = BufData::new();
  bufdata.parse_slice(&origlines).unwrap();
  curbuf.attach(&mut nvim, false, vec![]).expect("4");

  while let Ok(ChangedTickEvent { .. }) = main_from_handler.recv() {
    bufdata.resend_all_folds(&mut nvim).expect("5");
    curbuf.detach(&mut nvim).expect("6");
    nvim.command("call rpcnotify(1, 'quit')").unwrap();
  }
}
