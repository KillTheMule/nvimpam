//! This module contains all the global static [`Card`](::card::Card) instances
pub mod element;
pub mod node;
pub mod link;

pub use self::element::*;
/// All static declarations can be imported via
/// ```rust, ignore
/// use carddata::*;
/// ```
pub use self::node::*;
pub use self::link::*;
