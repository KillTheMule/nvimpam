//! This module contains all the global static [`Card`](::card::Card) instances
pub mod element;
pub mod link;
pub mod node;
pub mod part;

pub use self::element::*;
pub use self::link::*;
/// All static declarations can be imported via
/// ```rust, ignore
/// use carddata::*;
/// ```
pub use self::node::*;
pub use self::part::*;
