#![feature(test)]
extern crate neovim_lib;
extern crate nvimpam_lib;

#[macro_use]
extern crate criterion;
use criterion::Criterion;

use std::process::Command;
use std::sync::mpsc;

use nvimpam_lib::event::Event::*;
use nvimpam_lib::folds::FoldList;
use nvimpam_lib::handler::NeovimHandler;
use nvimpam_lib::lines::Lines;

use neovim_lib::neovim::Neovim;
use neovim_lib::neovim_api::NeovimApi;
use neovim_lib::session::Session;

fn bench_folds(c: &mut Criterion) {
  c.bench_function("integration1", |b| {
    let (sender, receiver) = mpsc::channel();
    let mut session = Session::new_child_cmd(
      Command::new("neovim/build/bin/nvim")
        .args(&["-u", "NONE", "--embed"])
        .env("VIMRUNTIME", "neovim/runtime"),
    ).unwrap();

    session.start_event_loop_handler(NeovimHandler(sender));
    let mut nvim = Neovim::new(session);

    nvim.command("noswap").expect("0");
    nvim.command("set rtp+=$PWD").expect("1");
    nvim.command("silent e! files/example.pc").expect("2");

    let curbuf = nvim.get_current_buf().expect("3");

    b.iter(|| {
      let mut foldlist = FoldList::new();
      let mut lines;
      curbuf.attach(&mut nvim, true, vec![]).expect("4");
      loop {
        match receiver.recv() {
          Ok(LinesEvent { linedata, .. }) => {
            lines = Lines::new(linedata);
            foldlist.recreate_all(&lines).expect("5");
            foldlist.resend_all(&mut nvim).expect("6");
            curbuf.detach(&mut nvim).expect("7");
            nvim.command("call rpcnotify(1, 'quit')").unwrap();
          }
          _ => break,
        }
      }
    })
  });
}

criterion_group!(integration, bench_folds);
criterion_main!(integration);
