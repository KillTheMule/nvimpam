//! This module contains all the global static [`Card`](::card::Card) instances
pub mod auxiliaries;
pub mod constraint;
pub mod element;
pub mod link;
pub mod node;
pub mod part;

pub use self::auxiliaries::*;
pub use self::constraint::*;
/// All static declarations can be imported via
/// ```rust, ignore
/// use nvimpam_lib::carddata::*;
/// ```
pub use self::element::*;
pub use self::link::*;
pub use self::node::*;
pub use self::part::*;
