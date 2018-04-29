//! The events that nvimpam needs to accept and deal with. They're sent by the
//! [`NeovimHandler`](handler/struct.NeovimHandler.html) to the main loop.
use failure::Error;
use std::fmt;
use std::sync::mpsc;

use neovim_lib::neovim::Neovim;
use neovim_lib::neovim_api::Buffer;
use neovim_lib::neovim_api::NeovimApi;

use folds::FoldList;
use lines::Lines;
use neovim_ext::BufferExt;

/// The event list the main loop reacts to
pub enum Event {
  /// Neovim's answer after sending registering for buffer events.
  /// `linedata` contains the buffers contents, without newlines. `more`
  /// indicates if we need to expect another event of this type with more
  /// lines, in case Neovim decided to split up the buffer (not yet
  /// implemented).
  UpdatesStart {
    buf: Buffer,
    changedtick: u64,
    linedata: Vec<String>,
    more: bool,
  },
  /// The update notification for a buffer change. Full lines only. Firstline
  /// is zero-indexed (i.e. a change on the first line will have `firstline =
  /// 0`).  If `numreplaced` is 0, the lines were added before `firstline`,
  /// but none were deleted.
  Update {
    buf: Buffer,
    changedtick: u64,
    firstline: u64,
    numreplaced: u64,
    linedata: Vec<String>,
  },
  /// Update notification for a new `changedtick` without a buffer change.
  /// Used by undo/redo.
  ChangedTick {
    buf: Buffer,
    changedtick: u64,
  },
  /// Notification the liveupdates are ending. Possible causes:
  ///  - Closing all a buffer's windows (unless 'hidden' is enabled).
  ///  - Using |:edit| to reload the buffer
  ///  - reloading the buffer after it is changed from outside neovim.
  UpdatesEnd { buf: Buffer },
  /// Recreate and resend the folds
  RefreshFolds,
  /// This plugin should quit. Currently only sent by the user directly.
  Quit,
}

impl Event {
  /// Run the event loop. The receiver receives the events from the
  /// [handler](../handler/struct.NeovimHandler.html).
  ///
  /// The loop starts by enabling
  /// [buffer events](../neovim_ext/trait.BufferExt.html#method.event_sub).
  /// It creates [`lines`](../lines/struct.Lines.html) and a
  /// [`foldlist`](../folds/struct.FoldList.html)  and updates them from the
  /// events received. It calls
  /// [`resend_all`](../folds/struct.FoldList.html#method.resend_all) when
  /// the [`foldlist`](../folds/struct.FoldList.html) was created, or the
  /// [`RefreshFolds`](../event/enum.Event.html#variant.RefreshFolds) event
  /// was sent.
  ///
  /// Sending the [`Quit`](../event/enum.Event.html#variant.Quit) event will
  /// exit the loop and return from the function.
  pub fn event_loop(
    receiver: &mpsc::Receiver<Event>,
    mut nvim: Neovim,
  ) -> Result<(), Error> {
    use self::Event::*;

    let curbuf = nvim.get_current_buf()?;
    curbuf.event_sub(&mut nvim, true)?;

    let mut foldlist = FoldList::new();
    let mut lines = Lines::new(Vec::new());

    loop {
      match receiver.recv() {
        Ok(UpdatesStart { linedata, .. }) => {
          lines = Lines::new(linedata);
          foldlist.recreate_all(&lines)?;
          foldlist.resend_all(&mut nvim)?;
        }
        Ok(Update {
          firstline,
          numreplaced,
          linedata,
          ..
        }) => {
          lines.update(firstline, numreplaced, linedata);
          foldlist.recreate_all(&lines)?;
        }
        Ok(RefreshFolds) => {
          foldlist.resend_all(&mut nvim)?;
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
      UpdatesStart {
        changedtick,
        ref linedata,
        more,
        ..
      } => write!(
        f,
        "UpdatesStart{{ changedtick: {}, #linedata: {}, \
         more: {} }}",
        changedtick,
        linedata.len(),
        more
      ),
      Update {
        changedtick,
        firstline,
        numreplaced,
        ref linedata,
        ..
      } => write!(
        f,
        "Update{{ changedtick: {}, firstline: {}, \
         numreplaced: {}, #linedata: {} }}",
        changedtick,
        firstline,
        numreplaced,
        linedata.len()
      ),
      ChangedTick {
        changedtick, ..
      } => write!(
        f,
        "ChangedTick{{ changedtick: {} }}",
        changedtick,
      ),
      UpdatesEnd { .. } => write!(f, "UpdatesEnd"),
      RefreshFolds => write!(f, "RefreshFolds"),
      Quit => write!(f, "Quit"),
    }
  }
}
