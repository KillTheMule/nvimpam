//! The nvimpam binary. Needs to be connected to neovim (currently,
//! https://github.com/neovim/neovim/pull/5269 is needed) by stdin/stdout like
//! this:
//!
//! ```text
//! let s:scriptdir = resolve(expand('<sfile>:p:h') . '/..')
//! let s:bin = s:scriptdir . '/nvimpam'
//! let s:id = jobstart([s:bin], { 'rpc': v:true }
//! ```
//!
//! It will automatically notify neovim of its activity, request the whole
//! buffer and parse it for folds. After that, it sends the folds to neovim.
//!
//! The source of nvimpam comes with the following files
//!
//! * init.vim
//! * autoload/nvimpam.vim
//! * plugin/nvimpam.vim
//!
//! Put the contents of `init.vim` into your `init.vim`, and move the other
//! files to their corresponding subfolders in your runtime path (check `:echo
//! $VIMRUNTIME` to find out). You will have the command `:NvimPamConnect` and
//! `:NvimPamStop` to start/stop the plugin.
//!
//! * TODO: Implement buffer updates
//! * TODO: Implement a function to reparse and recreate all folds
//! * TODO: Implement more card types than SHELL and NODE
//!
#[macro_use]
extern crate log;
extern crate simplelog;
extern crate neovim_lib;
extern crate nvimpam_lib;

use nvimpam_lib::handler::NeovimHandler;
use nvimpam_lib::event::Event;
use nvimpam_lib::folds::FoldList;
use nvimpam_lib::neovim_ext::BufferExt;
use nvimpam_lib::lines::Lines;

use std::error::Error;
use std::sync::mpsc;

use neovim_lib::neovim::Neovim;
use neovim_lib::neovim_api::NeovimApi;
use neovim_lib::session::Session;

// use log::SetLoggerError;
use simplelog::{Config, LogLevel, LogLevelFilter, WriteLogger};

fn main() {
  use std::process;

  match init_logging() {
    Err(e) => {
      eprintln!("Error initializing logger: {}", e);
      eprintln!("Nvimpam exiting!");
      process::exit(1);
    }
    Ok(()) => {}
  }

  match start_program() {
    Ok(_) => process::exit(0),

    Err(msg) => {
      eprintln!("Nvimpam encountered an error: {}", msg);
      eprintln!("Nvimpam exiting!");
      error!("{}", msg);
      process::exit(1);
    }
  };
}

fn init_logging() -> Result<(), Box<Error>> {
  use std::env;
  use std::fs::File;

  let filepath = match env::var_os("LOG_FILE") {
    Some(s) => s,
    None => return Ok(()),
  };

  let log_level = match env::var("LOG_LEVEL")
    .unwrap_or_else(|_| String::from("warn"))
    .to_lowercase()
    .as_ref() {
    "error" => LogLevelFilter::Error,
    "warn" => LogLevelFilter::Warn,
    "info" => LogLevelFilter::Info,
    "debug" => LogLevelFilter::Debug,
    "trace" => LogLevelFilter::Trace,
    _ => LogLevelFilter::Off,
  };

  let config = Config {
    time: Some(LogLevel::Error),
    level: Some(LogLevel::Error),
    target: Some(LogLevel::Error),
    location: Some(LogLevel::Error),
  };

  let log_file = File::create(filepath)?;
  WriteLogger::init(log_level, config, log_file)?;

  Ok(())
}

fn start_program() -> Result<(), Box<Error>> {

  let (sender, receiver) = mpsc::channel();
  let mut session = try!(Session::new_parent());

  session.start_event_loop_handler(NeovimHandler(sender));
  let mut nvim = Neovim::new(session);

  info!("let's notify neovim the plugin is connected!");
  nvim
    .command("echom \"rust client connected to neovim\"")
    .unwrap();
  info!("notification complete!");

  nvim.subscribe("quit").expect(
    "error: cannot subscribe to event: quit",
  );

  start_event_loop(&receiver, nvim);

  Ok(())
}


fn start_event_loop(receiver: &mpsc::Receiver<Event>, mut nvim: Neovim) {
  let curbuf = nvim.get_current_buf().unwrap();
  debug!("Before call");
  match curbuf.live_updates(&mut nvim, true) {
    Ok(o) => {
      debug!("curbuf.live_updates returned {:?}", o);
    }
    Err(e) => {
      error!("curbuf.liveupdates returned error: {:?}", e);
    }
  }
  debug!("after call");

  let mut foldlist = FoldList::new();
  let mut lines = Lines::new(Vec::new());

  loop {
    match receiver.recv() {
      Ok(Event::LiveUpdateStart { linedata, .. }) => {
        debug!("Running makeafold");
        lines = Lines::new(linedata);
        foldlist.recreate_all(&lines).unwrap();
        foldlist.resend_all(&mut nvim).unwrap();
        debug!("Makeafold ended");
      }
      Ok(Event::LiveUpdate { firstline, numreplaced, linedata, ..}) => {
        lines.update(firstline, numreplaced, linedata)
      }
      Ok(Event::Quit) => {
        debug!("Event::quit");
        break;
      }
      Ok(o) => {
        debug!("receiver recieved {:?}", o);
      }
      Err(e) => {
        debug!("receiver received error: {:?}", e);
      }
    }
  }
  info!("quitting");

}
