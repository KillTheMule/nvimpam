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

use std::{path::Path, process::Command, sync::mpsc};

use nvimpam_lib::{
  bufdata::BufData, card::keyword::Keywords, event::Event::*,
  handler::NeovimHandler, lines::Lines,
};

use neovim_lib::{neovim::Neovim, neovim_api::NeovimApi, session::Session};

fn main() {
  let (sender, receiver) = mpsc::channel();
  let nvimpath = Path::new("neovim").join("build").join("bin").join("nvim");

  let mut session = Session::new_child_cmd(
    Command::new(nvimpath)
      .args(&["-u", "NONE", "--embed"])
      .env("VIMRUNTIME", "neovim/runtime"),
  )
  .unwrap();

  session.start_event_loop_handler(NeovimHandler(sender));
  let mut nvim = Neovim::new(session);

  nvim.command("set noswapfile").expect("0");
  nvim.command("execute 'set rtp +='.getcwd()").expect("1");
  nvim.command("silent e! files/example.pc").expect("2");
  let curbuf = nvim.get_current_buf().expect("3");

  let mut foldlist = BufData::new();
  let origlines = Lines::read_file("files/example.pc").expect("3.1");
  let lines = Lines::from_slice(&origlines);
  let keywords = Keywords::from_lines(&lines);
  curbuf.attach(&mut nvim, false, vec![]).expect("4");

  while let Ok(ChangedTickEvent { .. }) = receiver.recv() {
    foldlist.recreate_all(&keywords, &lines).expect("5");
    foldlist.resend_all_folds(&mut nvim).expect("6");
    curbuf.detach(&mut nvim).expect("7");
    nvim.command("call rpcnotify(1, 'quit')").unwrap();
  }
}
