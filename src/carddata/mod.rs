pub mod element;
pub mod node;

pub use self::element::*;
/// All static declarations can be imported via
/// ```rust, ignore
/// use carddata::*;
/// ```
pub use self::node::*;
