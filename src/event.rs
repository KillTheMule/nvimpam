//! The events that nvimpam needs to accept and deal with. They're sent by the
//! [`NeovimHandler`](::handler::NeovimHandler) to the main loop.
use std::{ffi::OsString, fmt, sync::mpsc, fs};

use failure::{self, Error, ResultExt};

use neovim_lib::{
  neovim::Neovim,
  neovim_api::{Buffer, NeovimApi},
  Value,
};

use crate::bufdata::BufData;

/// The event list the main loop reacts to
pub enum Event {
  /// The update notification for a buffer change. Full lines only. Firstline
  /// is zero-indexed (i.e. a change on the first line will have `firstline =
  /// 0`). The range from firstline to lastline is end-exclusive. `more`
  /// indicates if we need to expect another event of this type with more
  /// lines, in case Neovim decided to split up the buffer (not yet
  /// implemented).
  LinesEvent {
    buf: Buffer,
    changedtick: u64,
    firstline: i64,
    lastline: i64,
    linedata: Vec<String>,
    more: bool,
  },
  /// Update notification for a new `changedtick` without a buffer change.
  /// Used by undo/redo.
  ChangedTickEvent { buf: Buffer, changedtick: u64 },
  /// Notification the liveupdates are ending. Possible causes:
  ///  - Closing all a buffer's windows (unless 'hidden' is enabled).
  ///  - Using |:edit| to reload the buffer
  ///  - reloading the buffer after it is changed from outside neovim.
  DetachEvent { buf: Buffer },
  /// Recreate and resend the folds
  RefreshFolds,
  /// Highlight lines in the buffer containing at least the given line range
  // TODO: maybe accept buffer as an argument?
  HighlightRegion { firstline: u64, lastline: u64 },
  /// This plugin should quit. Currently only sent by the user directly.
  Quit,
}

impl Event {
  /// Run the event loop. The receiver receives the events from the
  /// [handler](::handler::NeovimHandler).
  ///
  /// If a file was given as an argument, nvimpam reads it and creates its
  /// [`BufData`](::bufdata::BufData) from it. Then it enables
  /// [buffer events](https://neovim.io/doc/user/api.html#nvim_buf_attach()) and
  /// updates the [`BufData`](::bufdata::BufData) accordingly.
  ///
  /// If no file was given as an argument, nvimpam directly enables
  /// [buffer events](https://neovim.io/doc/user/api.html#nvim_buf_attach())
  /// and requests the buffer's contents from it instead.
  ///
  /// Sending the [`Quit`](::event::Event::Quit) event will
  /// exit the loop and return from the function.
  pub fn event_loop(
    from_handler: &mpsc::Receiver<Event>,
    to_handler: &mpsc::Sender<Value>,
    nvim: &mut Neovim,
    file: Option<OsString>,
  ) -> Result<(), Error> {
    use self::Event::*;

    let curbuf = nvim
      .get_current_buf()
      .context("Event loop could not get current buffer")?;

    let origlines;
    let mut bufdata = BufData::new();

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
          if lastline == -1 {
            bufdata.parse_vec(linedata)?;
          } else if lastline >= 0 && firstline >= 0 {
            let (start, end) =
              bufdata.update(firstline as u64, lastline as u64, linedata)?;

            crate::bufdata::highlights::highlight_region(
              bufdata.highlights.indexrange(start, end),
              nvim,
              firstline as u64,
              lastline as u64,
            )?;
          } else {
            error!(
              "LinesEvent only works with nonnegative numbers, except for
               lastline = -1!"
            );
          }
        }
        Ok(RefreshFolds) => to_handler.send(bufdata.packup_all_folds())?,
        Ok(HighlightRegion {
          firstline,
          lastline,
        }) => {
          let fl = bufdata.keywords.first_before(firstline);
          let mut ll = bufdata.keywords.first_after(lastline);

          // highlight_region is end_exclusive, so we need to make sure
          // we include the last line requested even if it is a keyword line
          if ll == lastline as usize {
            ll += 1;
          }

          crate::bufdata::highlights::highlight_region(
            bufdata.highlights.linerange(fl as u64, ll as u64),
            nvim,
            fl as u64,
            ll as u64,
          )?;
        }
        Ok(Quit) => {
          break;
        }
        Ok(o) => {
          warn!("receiver recieved {:?}", o);
        }
        Err(e) => {
          warn!("receiver received error: {:?}", e);
        }
      }
    }
    info!("quitting");
    Ok(())
  }
}

impl fmt::Debug for Event {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    use self::Event::*;

    match *self {
      LinesEvent {
        changedtick,
        firstline,
        lastline,
        ref linedata,
        ..
      } => write!(
        f,
        "Update{{ changedtick: {}, firstline: {}, \
         lastline: {}, #linedata: {} }}",
        changedtick,
        firstline,
        lastline,
        linedata.len()
      ),
      ChangedTickEvent { changedtick, .. } => {
        write!(f, "ChangedTick{{ changedtick: {} }}", changedtick,)
      }
      HighlightRegion {
        firstline,
        lastline,
      } => write!(
        f,
        "HighlightRegion{{ firstline: {}, lastline: {} }}",
        firstline, lastline
      ),
      DetachEvent { .. } => write!(f, "DetachEvent"),
      RefreshFolds => write!(f, "RefreshFolds"),
      Quit => write!(f, "Quit"),
    }
  }
}
