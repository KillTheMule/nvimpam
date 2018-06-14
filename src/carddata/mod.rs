//! This module contains all the global static [`Card`](::card::Card) instances
pub mod element;
pub mod link;
pub mod node;
pub mod part;
pub mod constraint;
pub mod auxiliaries;

/// All static declarations can be imported via
/// ```rust, ignore
/// use carddata::*;
/// ```
pub use self::element::*;
pub use self::link::*;
pub use self::node::*;
pub use self::part::*;
pub use self::constraint::*;
pub use self::auxiliaries::*;
