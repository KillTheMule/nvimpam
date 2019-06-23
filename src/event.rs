//! The events that nvimpam needs to accept and deal with. They're sent by the
//! [`NeovimHandler`](crate::handler::NeovimHandler) to the main loop.
use std::fmt;

use neovim_lib::neovim_api::Buffer;

use crate::linenr::LineNr;

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
    firstline: LineNr,
    lastline: LineNr,
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
  HighlightRegion { firstline: LineNr, lastline: LineNr },
  /// Request the CellHint at the given cursor position
  CellHint { line: LineNr, column: u8 },
  /// Add a comment with hints above a line
  CommentLine { line: LineNr },
  /// Return an end-inclusive range start..=end of lines in which the card of
  /// the current line is included
  CardRange { line: LineNr },
  /// Return a String with the line having all cells aligned, or nil if the
  /// line was aligned, is a comment, or otherwise non-aligneable
  AlignLine { line: LineNr },
  /// This plugin should quit. Currently only sent by the user directly.
  Quit,
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
      CellHint { line, column } => {
        write!(f, "CellHint{{ line: {}, column: {} }}", line, column)
      }
      CommentLine { line } => write!(f, "CommentLine{{ line: {} }}", line),
      CardRange { line } => write!(f, "CardRange{{ line: {} }}", line),
      AlignLine { line } => write!(f, "AlignLine{{ line: {} }}", line),
      DetachEvent { .. } => write!(f, "DetachEvent"),
      RefreshFolds => write!(f, "RefreshFolds"),
      Quit => write!(f, "Quit"),
    }
  }
}
