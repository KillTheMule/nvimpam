//! The events that nvimpam needs to accept and deal with. They're sent by the
//! [`NeovimHandler`](::handler::NeovimHandler) to the main loop.
use std::{ffi::OsString, fmt, sync::mpsc};

use failure::{self, Error};

use neovim_lib::{
  neovim::Neovim,
  neovim_api::{Buffer, NeovimApi},
};

use crate::{bufdata::BufData, lines::Lines};

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
  /// The loop starts by enabling
  /// [buffer events](https://neovim.io/doc/user/api.html#nvim_buf_attach()).
  /// It creates [`lines`](::lines::Lines),
  /// [`keywords`](::card::keyword::Keywords) and a
  /// [`foldlist`](::folds::FoldList)  and updates them from the events
  /// received. It calls [`resend_all`](::folds::FoldList::resend_all) when the
  /// [`foldlist`](::folds::FoldList) was created, or the
  /// [`RefreshFolds`](../event/enum.Event.html#variant.RefreshFolds) event was
  /// sent.
  ///
  /// Sending the [`Quit`](../event/enum.Event.html#variant.Quit) event will
  /// exit the loop and return from the function.
  pub fn event_loop(
    receiver: &mpsc::Receiver<Event>,
    mut nvim: Neovim,
    file: Option<OsString>,
  ) -> Result<(), Error> {
    use self::Event::*;

    let curbuf = nvim.get_current_buf()?;

    let origlines;
    let mut bufdata = BufData::new();

    let connected = match file {
      None => curbuf.attach(&mut nvim, true, vec![])?,
      Some(f) => {
        origlines = Lines::read_file(f)?;
        bufdata.from_slice(&origlines);
        bufdata.resend_all_folds(&mut nvim)?;
        curbuf.attach(&mut nvim, false, vec![])?
      }
    };

    if !connected {
      return Err(failure::err_msg("Could not enable buffer updates!"));
    }

    loop {
      match receiver.recv() {
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
            bufdata.from_vec(linedata);
            bufdata.resend_all_folds(&mut nvim)?;
          } else if lastline >= 0 && firstline >= 0 {
            bufdata.update(firstline as u64, lastline as u64, linedata);

            /*
            crate::bufdata::highlights::highlight_region(
              tmp_bufdata.highlights.iter(),
              &mut nvim,
              first as u64,
              last as u64,
              true,
            )?;
            */
          } else {
            error!(
              "LinesEvent only works with nonnegative numbers, except for
               lastline = -1!"
            );
          }
        }
        Ok(RefreshFolds) => {
          bufdata.resend_all_folds(&mut nvim)?;
        }
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
            &mut nvim,
            fl as u64,
            ll as u64,
            false,
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
        "Hl_Line{{ firstline: {}, lastline: {} }}",
        firstline, lastline
      ),
      DetachEvent { .. } => write!(f, "UpdatesEnd"),
      RefreshFolds => write!(f, "RefreshFolds"),
      Quit => write!(f, "Quit"),
    }
  }
}
