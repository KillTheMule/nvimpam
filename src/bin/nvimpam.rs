//! The nvimpam binary. Needs to be connected to neovim by stdin/stdout like
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
//! If you want logging, set the following environment variables:
//!
//! * `LOG_FILE` is the path to the log file (no logging if this is empty)
//! * `LOG_LEVEL` can be one of `error`, `warn`, `info`, `debug` and `trace`, in
//!    ascending order of verbosity. The default is `warn`.
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
use neovim_lib::Value;

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

fn send_client_info(nvim: &mut Neovim) -> Result<(), Error> {
  const VERSION_MAJOR: &str = env!("CARGO_PKG_VERSION_MAJOR");
  const VERSION_MINOR: &str = env!("CARGO_PKG_VERSION_MINOR");
  const VERSION_PATCH: &str = env!("CARGO_PKG_VERSION_PATCH");
  const VERSION_PRE: &str = env!("CARGO_PKG_VERSION_PRE");
  const NAME: &str = env!("CARGO_PKG_NAME");

  let version: Vec<(Value, Value)> = vec![
    ("major".into(), VERSION_MAJOR.into()),
    ("minor".into(), VERSION_MINOR.into()),
    ("patch".into(), VERSION_PATCH.into()),
    ("prerelease".into(), VERSION_PRE.into()),
  ];

  let methods: Vec<(Value, Value)> = vec![
    (
      "quit".into(),
      vec![Value::from("nargs"), Value::from(0u8)].into(),
    ),
    (
      "RefreshFolds".into(),
      vec![Value::from("nargs"), Value::from(0u8)].into(),
    ),
  ];

  let attribs: Vec<(Value, Value)> = vec![
    ("license".into(), "Apache-2.0 OR MIT".into()),
    (
      "documentation".into(),
      "https://KillTheMule.github.io/nvimpam/nvimpam".into(),
    ),
    (
      "repository".into(),
      "https://github.com/KillTheMule/nvimpam".into(),
    ),
    (
      "author".into(),
      "KillTheMule <KillTheMule@users.noreply.github.com".into(),
    ),
  ];

  let typ = "remote";
  nvim.set_client_info(NAME, version, typ, methods, attribs)?;
  Ok(())
}

fn start_program() -> Result<(), Error> {
  let (sender, receiver) = mpsc::channel();
  let mut session = try!(Session::new_parent());

  session.start_event_loop_handler(NeovimHandler(sender));
  let mut nvim = Neovim::new(session);

  send_client_info(&mut nvim)?;

  nvim
    .command("echom \"rust client connected to neovim\"")
    .context("Could not 'echom' to neovim")?;

  nvim
    .subscribe("quit")
    .context("error: cannot subscribe to event: quit")?;

  Event::event_loop(&receiver, nvim)?;

  Ok(())
}
