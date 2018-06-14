//! The events that nvimpam needs to accept and deal with. They're sent by the
//! [`NeovimHandler`](::handler::NeovimHandler) to the main loop.
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
  /// This plugin should quit. Currently only sent by the user directly.
  Quit,
}

impl Event {
  /// Run the event loop. The receiver receives the events from the
  /// [handler](../handler/struct.NeovimHandler.html).
  ///
  /// The loop starts by enabling
  /// [buffer events](::neovim_ext::BufferExt::attach).
  /// It creates [`lines`](::lines::Lines) and a
  /// [`foldlist`](::folds::FoldList)  and updates them from the
  /// events received. It calls
  /// [`resend_all`](::folds::FoldList::resend_all) when
  /// the [`foldlist`](::folds::FoldList) was created, or the
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
    curbuf.attach(&mut nvim, true, vec![])?;

    let mut foldlist = FoldList::new();
    let mut lines = Lines::new(Vec::new());

    loop {
      match receiver.recv() {
        Ok(LinesEvent {
          firstline,
          lastline,
          linedata,
          ..
        }) => {
          if lastline == -1 {
            lines = Lines::new(linedata);
            foldlist.recreate_all(&lines)?;
            foldlist.resend_all(&mut nvim)?;
          } else if lastline >= 0 && firstline >= 0 {
            lines.update(firstline as usize, lastline as usize, linedata);
            foldlist.recreate_all(&lines)?;
          } else {
            error!(
              "LinesEvent only works with nonnegative numbers, except for
               lastline = -1!"
            );
          }
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
      DetachEvent { .. } => write!(f, "UpdatesEnd"),
      RefreshFolds => write!(f, "RefreshFolds"),
      Quit => write!(f, "Quit"),
    }
  }
}
