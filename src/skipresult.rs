//! This module holds datastructures for the various `skip_*` methods of the
//! [`NoCommentIter`](::nocommentiter::NoCommentIter)
use std::fmt;

use crate::lines::ParsedLine;

/// A struct returned by
/// [`skip_to_next_keyword`](::nocommentiter::NoCommentIter::
/// skip_to_next_keyword).

/// A data structure returned by several skip methods on
/// [`NoCommentIter`](::nocommentiter::NoCommentIter)
///
/// `nextline` will be `None` in those cases where the iterator returned `None`
/// before such a line could be found, i.e. the file ended.
///
/// `skip_end` is the index of the last line we skipped.
#[derive(Debug)]
pub struct SkipResult<'a> {
  pub nextline: Option<ParsedLine<'a>>,
  pub skip_end: i64,
}

impl<'a> fmt::Display for SkipResult<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    if let Some(ref pl) = self.nextline {
      write!(
        f,
        "SkipResult {{ nextline: {}, skip_end: {:?} }}",
        pl, self.skip_end
      )
    } else {
      write!(
        f,
        "SkipResult {{ nextline: None, skip_end: {:?} }}",
        self.skip_end
      )
    }
  }
}
