//! The nvimpam binary. Needs to be connected to neovim (currently,
//! <https://github.com/neovim/neovim/pull/5269> is needed) by stdin/stdout like
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
//! $VIMRUNTIME` to find out). You will have the commands `:NvimPamConnect` and
//! `:NvimPamStop` to start/stop the plugin,as well as `:NvimPamUpdateFolds` to
//! recompute and update all folds.
//!
//! If you want logging, set the following environment variables:
//!
//! * `LOG_FILE` is the path to the log file (no logging if this is empty)
//! * `LOG_LEVEL` can be one of `error`, `warn`, `info`, `debug` and `trace`, in
//!    ascending order of verbosity. The default is `warn`.
//!
//! `TODO`: Implement more card types than SHELL, NODE and Comment
//!
#[macro_use]
extern crate log;
extern crate failure;
extern crate neovim_lib;
extern crate nvimpam_lib;
extern crate simplelog;

use std::sync::mpsc;

use failure::Error;
use failure::ResultExt;

use nvimpam_lib::event::Event;
use nvimpam_lib::handler::NeovimHandler;

use neovim_lib::neovim::Neovim;
use neovim_lib::neovim_api::NeovimApi;
use neovim_lib::session::Session;

// use log::SetLoggerError;
use simplelog::{Config, Level, LevelFilter, WriteLogger};

fn main() {
  use std::process;

  match init_logging() {
    Err(e) => {
      eprintln!("Nvimpam: Error initializing logger: {}", e);
      error!("Error initializing logger: {}", e);
      for cause in e.causes() {
        error!("Caused by: {}", cause)
      }
      error!("Nvimpam exiting!");
      process::exit(1);
    }
    Ok(()) => {}
  }

  match start_program() {
    Ok(_) => process::exit(0),
    Err(e) => {
      eprintln!("Nvimpam encountered an error: {}", e);
      error!("Nvimpam encountered an error: {}", e);
      for cause in e.causes() {
        error!("Caused by: {}", cause)
      }
      error!("Nvimpam exiting!");
      process::exit(1);
    }
  };
}

fn init_logging() -> Result<(), Error> {
  use std::env;
  use std::env::VarError;
  use std::fs::File;

  let filepath = match env::var_os("LOG_FILE") {
    Some(s) => s,
    None => return Ok(()),
  };

  let log_level = match env::var("LOG_LEVEL") {
    Ok(s) => s,
    Err(VarError::NotPresent) => "warn".to_owned(),
    e @ Err(VarError::NotUnicode(_)) => {
      e.context("'LOG_LEVEL' not UTF-8 compatible!")?
    }
  };

  let log_level = match log_level.to_lowercase().as_ref() {
    "error" => LevelFilter::Error,
    "warn" => LevelFilter::Warn,
    "info" => LevelFilter::Info,
    "debug" => LevelFilter::Debug,
    "trace" => LevelFilter::Trace,
    _ => LevelFilter::Off,
  };

  let config = Config {
    time: Some(Level::Error),
    level: Some(Level::Error),
    target: Some(Level::Error),
    location: Some(Level::Error),
    time_format: Some("%+"),
  };

  let log_file = File::create(filepath)?;
  WriteLogger::init(log_level, config, log_file)?;

  Ok(())
}

fn start_program() -> Result<(), Error> {
  let (sender, receiver) = mpsc::channel();
  let mut session = try!(Session::new_parent());

  session.start_event_loop_handler(NeovimHandler(sender));
  let mut nvim = Neovim::new(session);

  nvim
    .command("echom \"rust client connected to neovim\"")
    .context("Could not 'echom' to neovim")?;

  nvim
    .subscribe("quit")
    .context("error: cannot subscribe to event: quit")?;

  Event::event_loop(&receiver, nvim)?;

  Ok(())
}
