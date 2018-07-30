//! This module holds datastructures for the various `skip_*` methods of the
//! [`NoCommentIter`](::nocommentiter::NoCommentIter)
use std::fmt;
use std::convert::From;

use card::keyword::Keyword;

/// A struct returned by [`NoCommentIter`](::nocommentiter::NoCommentIter).
#[derive(PartialEq)]
pub struct ParsedLine<'a, T: 'a>
where
  T: AsRef<str>,
{
  pub number: usize,
  pub text: &'a T,
  pub keyword: Option<Keyword>,
}

impl<'a, T> ParsedLine<'a, T> where T: AsRef<str> {
  pub fn try_into_keywordline(self) -> Option<KeywordLine<'a, T>> {
    if let Some(kw) = self.keyword {
      return Some(KeywordLine { number: self.number, text: self.text, keyword: kw })
    } else {
      return None
    }
  }
}

impl<'a, T> From<KeywordLine<'a, T>> for ParsedLine<'a,T>
where T: AsRef<str> {
  fn from(k: KeywordLine<'a, T>) -> ParsedLine<'a, T> {
    ParsedLine { number: k.number, text: k.text, keyword:
      Some(k.keyword) }
  }
}

impl<'a, T: 'a> fmt::Debug for ParsedLine<'a, T>
where
  T: AsRef<str>,
{
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "ParsedLine {{ number: {:?}, text: {:?}, keyword: {:?} }}",
      self.number,
      self.text.as_ref(),
      self.keyword
    )
  }
}

/// A struct returned by
/// [`skip_to_next_keyword`](::nocommentiter::NoCommentIter::
/// skip_to_next_keyword).
#[derive(PartialEq)]
pub struct KeywordLine<'a, T: 'a>
where
  T: AsRef<str>,
{
  pub number: usize,
  pub text: &'a T,
  pub keyword: Keyword,
}

impl<'a, T: 'a> fmt::Debug for KeywordLine<'a, T>
where
  T: AsRef<str>,
{
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "ParsedLine {{ number: {:?}, text: {:?}, keyword: {:?} }}",
      self.number,
      self.text.as_ref(),
      self.keyword
    )
  }
}

/// A data structure returned by several skip methods on
/// [`NoCommentIter`](::nocommentiter::NoCommentIter)
///
/// `nextline` will be `None` in those cases where the iterator returned `None`
/// before such a line could be found, i.e. the file ended.
///
/// `skip_end` is the index of the last line we skipped. It will be `None` if
/// we could not fully skip something before the file ended7416
#[derive(Debug)]
pub struct SkipResult<'a, T: 'a>
where
  T: AsRef<str>,
{
  pub nextline: Option<ParsedLine<'a, T>>,
  pub skip_end: Option<usize>,
}

impl<'a, T: 'a> Default for SkipResult<'a, T>
where
  T: AsRef<str>,
{
  fn default() -> Self {
    SkipResult {
      nextline: None,
      skip_end: None,
    }
  }
}
