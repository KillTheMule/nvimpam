//! This module holds the returned datastructure for the various `skip_*`
//! methods of the [`NoCommentIter`](::nocommentiter::NoCommentIter)
use std::fmt;

use card::keyword::Keyword;

/// A data structure returned by several skip methods on
/// [`NoCommentIter`](::nocommentiter::NoCommentIter)
///
/// `nextline` is a tuple for the next line to be processed, i.e. the last line
/// the iterator returned. The tuple consists of the index and the line itself.
/// It will be `None` in those cases where the iterator returned `None` before
/// such a line could be found, i.e. the file ended.
///
/// `skip_end` is the index of the last line we skipped. It will be `None` if
/// we could not fully skip something before the file ended
pub struct SkipResult<'a, T: 'a>
where
  T: AsRef<str>,
{
  pub nextline: Option<(usize, &'a T)>,
  pub nextline_kw: Option<Keyword>,
  pub skip_end: Option<usize>,
}

impl<'a, T: 'a> Default for SkipResult<'a, T>
where
  T: AsRef<str>,
{
  fn default() -> Self {
    SkipResult {
      nextline: None,
      nextline_kw: None,
      skip_end: None,
    }
  }
}

impl<'a, T: 'a> fmt::Debug for SkipResult<'a, T>
where
  T: AsRef<str>,
{
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    if self.nextline.is_none() {
      write!(
        f,
        "SkipResult {{ nextline: None, nextline_kw: {:?}, skip_end: {:?} }}",
        self.nextline_kw, self.skip_end
      )
    } else {
      let n = self.nextline.unwrap();
      write!(
        f,
        "SkipResult {{ nextline: ({:?}, {:?}), nextline_kw: {:?}, \
         skip_end: {:?} }}",
        n.0,
        n.1.as_ref(),
        self.nextline_kw,
        self.skip_end
      )
    }
  }
}
