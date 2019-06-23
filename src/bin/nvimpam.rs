#![deny(bare_trait_objects)]
//! The nvimpam binary. Needs to be connected to neovim by stdin/stdout like
//! this (assuming it is in your PATH):
//!
//! ```text
//! let s:bin = 'nvimpam'
//! let s:id = jobstart([s:bin], { 'rpc': v:true }
//! ```
//!
//! It will automatically notify neovim of its activity, request the whole
//! buffer and parse it for folds. After that, it sends the folds to neovim.
//!
//! As a performance optimization, pass the filename as a first argument:
//!
//! ```text
//! let s:bin = 'nvimpam'
//! let s:file = expand('%:p')
//! let s:id = jobstart([s:bin.' '.s:file], { 'rpc': v:true }
//! ```
//!
//! Nvimpam will load the file from disc instead of requesting it over RPC,
//! which is quite a bit faster (mostly probably because the file is cached by
//! your OS since it was loaded by neovim just before).
//!
//! If you want logging, set the following environment variables:
//!
//! * `NVIMPAM_LOG_FILE` is the path to the log file (no logging if this is
//!   empty)
//! * `NVIMPAM_LOG_LEVEL` can be one of `error`, `warn`, `info`, `debug` and
//!   `trace`, in ascending order of verbosity. The default is `warn`.
use std::{env::args_os, ffi::OsString, fs, sync::mpsc};

use failure::{Error, ResultExt};
use log::{error, info, warn};
use neovim_lib::{
  neovim::Neovim, neovim_api::NeovimApi, session::Session, Value,
};
use simplelog::{Config, Level, LevelFilter, WriteLogger};

use nvimpam_lib::{bufdata::BufData, event::Event, handler::NeovimHandler};

fn main() {
  use std::process;

  match init_logging() {
    Err(e) => {
      eprintln!("Nvimpam: Error initializing logger: {}", e);
      for cause in e.iter_chain().skip(1) {
        eprintln!("Caused by: {}", cause)
      }
      eprintln!("Nvimpam exiting!");
      process::exit(1);
    }
    Ok(()) => {}
  }

  match start_program() {
    Ok(_) => process::exit(0),
    Err(e) => {
      error!("Nvimpam encountered an error: {}", e);
      for cause in e.iter_chain().skip(1) {
        error!("Caused by: {}", cause)
      }
      error!("Nvimpam exiting!");
      process::exit(1);
    }
  };
}

fn send_err(nvim: &mut Neovim, err: &Error) {
  let luafn = "require('nvimpam').nvimpam_err(...)";
  let luaargs = Value::from(format!("Nvimpam ecountered an error: {:?}!", err));

  if let Err(e) = nvim.execute_lua(luafn, vec![luaargs]) {
    error!(
      "Could not send error to neovim: '{:?}'.\n Original error was: '{:?}'",
      e, err
    );
  }
}

fn init_logging() -> Result<(), Error> {
  use std::{
    env::{self, VarError},
    fs::File,
  };

  let filepath = match env::var_os("NVIMPAM_LOG_FILE") {
    Some(s) => s,
    None => return Ok(()),
  };

  let log_level = match env::var("NVIMPAM_LOG_LEVEL") {
    Ok(s) => s,
    Err(VarError::NotPresent) => "warn".to_owned(),
    e @ Err(VarError::NotUnicode(_)) => {
      e.context("'NVIMPAM_LOG_LEVEL' not UTF-8 compatible!")?
    }
  };

  let log_level = match log_level.to_lowercase().as_ref() {
    "error" => LevelFilter::Error,
    "warn" => LevelFilter::Warn,
    "info" => LevelFilter::Info,
    "debug" => LevelFilter::Debug,
    "trace" => LevelFilter::Trace,
    _ => {
      eprintln!(
        "NVIMPAM_LOG_LEVEL (='{}') unknown, disabling logging!",
        log_level
      );
      LevelFilter::Off
    }
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
      vec![Value::from(vec![Value::from("nargs"), Value::from(0_u8)])].into(),
    ),
    (
      "RefreshFolds".into(),
      vec![
        Value::from(vec![Value::from("nargs"), Value::from(0_u8)]),
        Value::from(vec![Value::from("async"), Value::from(false)]),
      ]
      .into(),
    ),
    (
      "HighlightRegion".into(),
      vec![
        Value::from(vec![Value::from("nargs"), Value::from(2_u8)]),
        Value::from(vec![Value::from("async"), Value::from(true)]),
      ]
      .into(),
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
  let (handler_to_main, main_from_handler) = mpsc::channel();
  let (main_to_handler, handler_from_main) = mpsc::channel();
  let mut session = Session::new_parent()?;

  session.start_event_loop_handler(NeovimHandler {
    to_main: handler_to_main,
    from_main: handler_from_main,
  });
  let mut nvim = Neovim::new(session);

  send_client_info(&mut nvim)?;

  let file = args_os().nth(1);

  event_loop(&main_from_handler, &main_to_handler, &mut nvim, file).map_err(
    |e| {
      send_err(&mut nvim, &e);
      e
    },
  )
}

/// Run the event loop. The receiver receives the events from the
/// [handler](crate::handler::NeovimHandler).
///
/// If a file was given as an argument, nvimpam reads it and creates its
/// [`BufData`](crate::bufdata::BufData) from it. Then it enables
/// [buffer events](https://neovim.io/doc/user/api.html#nvim_buf_attach()) and
/// updates the [`BufData`](crate::bufdata::BufData) accordingly.
///
/// If no file was given as an argument, nvimpam directly enables
/// [buffer events](https://neovim.io/doc/user/api.html#nvim_buf_attach())
/// and requests the buffer's contents from it instead.
///
/// Sending the [`Quit`](crate::event::Event::Quit) event will
/// exit the loop and return from the function.
pub fn event_loop(
  from_handler: &mpsc::Receiver<Event>,
  to_handler: &mpsc::Sender<Value>,
  nvim: &mut Neovim,
  file: Option<OsString>,
) -> Result<(), Error> {
  use Event::*;

  let curbuf = nvim.get_current_buf()?;
  let origlines;
  let mut bufdata = BufData::new(&curbuf);

  let connected = match file {
    None => curbuf.attach(nvim, true, vec![])?,
    Some(f) => {
      origlines = fs::read(f)?;
      bufdata.parse_slice(&origlines)?;
      curbuf.attach(nvim, false, vec![])?
    }
  };

  if !connected {
    return Err(failure::err_msg("Could not enable buffer updates!"));
  }

  loop {
    match from_handler.recv() {
      Ok(LinesEvent {
        firstline,
        lastline,
        linedata,
        changedtick,
        ..
      }) => {
        if changedtick == 0 {
          continue;
        }
        let hlrange = bufdata.update(firstline, lastline, linedata)?;

        if let Some(calls) = bufdata.highlight_region_calls(hlrange) {
          nvim.call_atomic(calls).context("call_atomic failed")?;
        }
      }
      Ok(RefreshFolds) => to_handler.send(bufdata.fold_calls())?,
      Ok(HighlightRegion {
        firstline,
        lastline,
      }) => {
        // Note to self: This returns the index range of the highlights, not
        // the lines
        let hlrange = bufdata.hl_linerange(firstline, lastline);

        if let Some(calls) = bufdata.highlight_region_calls(hlrange) {
          nvim.call_atomic(calls).context("call_atomic failed")?;
        }
      }
      Ok(CellHint { line, column }) => {
        to_handler.send(bufdata.cellhint(line, column))?;
      }

      Ok(CommentLine { line }) => {
        to_handler.send(bufdata.linecomment(line))?;
      }
      Ok(CardRange { line }) => {
        to_handler.send(bufdata.cardrange(line))?;
      }
      Ok(AlignLine { line }) => {
        to_handler.send(bufdata.align_line(line))?;
      }
      Ok(Quit) => {
        break;
      }
      Ok(DetachEvent { buf }) => {
        if *bufdata.buf == buf {
          buf
            .clear_namespace(nvim, 5, 0, -1)
            .context("could not clear namespace 5")?;
          break;
        } else {
          warn!(
            "Received Detach Event for buffer {:?}, but was attached to
             buffer {:?}. Continuing!",
            buf, bufdata.buf
          );
        }
      }
      Ok(o) => {
        warn!("receiver recieved unkown event {:?}", o);
      }
      Err(e) => {
        warn!("receiver received error: {:?}", e);
      }
    }
  }
  info!("quitting");
  Ok(())
}
