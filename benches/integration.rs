extern crate neovim_lib;
extern crate nvimpam_lib;

#[macro_use]
extern crate criterion;
use criterion::Criterion;

use std::{path::Path, process::Command, sync::mpsc};

use nvimpam_lib::{
  bufdata::BufData, card::keyword::Keywords, event::Event::*,
  handler::NeovimHandler, lines::Lines,
};

use neovim_lib::{neovim::Neovim, neovim_api::NeovimApi, session::Session};

fn bench_folds(c: &mut Criterion) {
  let (sender, receiver) = mpsc::channel();
  let nvimpath = Path::new("neovim").join("build").join("bin").join("nvim");

  let mut session = Session::new_child_cmd(
    Command::new(nvimpath)
      .args(&["-u", "NONE", "--embed"])
      .env("VIMRUNTIME", "neovim/runtime"),
  )
  .unwrap();
  session.set_infinity_timeout();

  session.start_event_loop_handler(NeovimHandler(sender));
  let mut nvim = Neovim::new(session);

  nvim.command("set noswapfile").expect("0");
  nvim.command("execute 'set rtp +='.getcwd()").expect("1");
  nvim.command("silent e!  files/example.pc").expect("2");
  let curbuf = nvim.get_current_buf().expect("3");

  c.bench_function("integration1", move |b| {
    b.iter(|| {
      let mut foldlist = BufData::new();
      let mut lines;
      let mut keywords: Keywords;
      curbuf.attach(&mut nvim, true, vec![]).expect("4");
      loop {
        match receiver.recv() {
          Ok(LinesEvent { linedata, .. }) => {
            lines = Lines::from_vec(linedata);
            keywords = Keywords::from_lines(&lines);
            foldlist.recreate_all(keywords.as_ref(), &lines).expect("5");
            foldlist.resend_all_folds(&mut nvim).expect("6");
            curbuf.detach(&mut nvim).expect("7");
            nvim.command("call rpcnotify(1, 'quit')").unwrap();
          }
          _ => break,
        }
      }
    })
  });
}

criterion_group!(name = integration; config = Criterion::default().sample_size(10).without_plots(); targets = bench_folds);
criterion_main!(integration);
