//! The events that nvimpam needs to accept and deal with. They're sent by the
//! [NeovimHandler](handler/struct.NeovimHandler.html) to the main loop.
use std::fmt;
use neovim_lib::neovim_api::Buffer;

/// The event list the main loop reacts to
pub enum Event {
  /// Neovim's answer after sending live_updates(true) for a buffer.
  /// `linedata` contains the buffers contents, without newlines. `more`
  /// indicates if we need to expect another event of this type with more
  /// lines, in case Neovim decided to split up the buffer (not yet
  /// implemented).
  LiveUpdateStart {
    buf: Buffer,
    changedtick: u64,
    linedata: Vec<String>,
    more: bool,
  },
  /// The update notification for a buffer change. Full lines only. Firstline
  /// is zero-indexed (i.e. a change on the first line will have `firstline =
  /// 0`).  If `numreplaced` is 0, the lines were added before `firstline`,
  /// but none were deleted.
  LiveUpdate {
    buf: Buffer,
    changedtick: u64,
    firstline: u64,
    numreplaced: u64,
    linedata: Vec<String>,
  },
  /// Update notification for a new `changedtick` without a buffer change.
  /// Used by undo/redo.
  LiveUpdateTick { buf: Buffer, changedtick: u64 },
  /// Notification the liveupdates are ending. Possible causes:
  ///  - Closing all a buffer's windows (unless 'hidden' is enabled).
  ///  - Using |:edit| to reload the buffer
  ///  - reloading the buffer after it is changed from outside neovim.
  LiveUpdateEnd { buf: Buffer },
  /// This plugin should quit. Currently only sent by the user directly.
  Quit,
}

impl fmt::Debug for Event {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    use self::Event::*;

    match *self {
      LiveUpdateStart {
        changedtick,
        ref linedata,
        more,
        ..
      } => {
        write!(
          f,
          "LiveUpdateStart{{ changedtick: {}, #linedata: {}, \
                                       more: {} }}",
          changedtick,
          linedata.len(),
          more
        )
      }
      LiveUpdate {
        changedtick,
        firstline,
        numreplaced,
        ref linedata,
        ..
      } => {
        write!(
          f,
          "LiveUpdate{{ changedtick: {}, firstline: {}, \
                                  numreplaced: {}, #linedata: {} }}",
          changedtick,
          firstline,
          numreplaced,
          linedata.len()
        )
      }
      LiveUpdateTick { changedtick, .. } => {
        write!(
                    f,
                    "LiveUpdateTick{{ changedtick: {} }}",
                    changedtick,
                )
      }
      LiveUpdateEnd { .. } => write!(f, "LiveUpdateEnd"),
      Quit => write!(f, "Quit"),
    }
  }
}
