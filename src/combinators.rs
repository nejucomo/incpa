//! Types for composed parsers, typically instantiated through [Parser](crate::Parser) methods
mod maperror;
mod mapoutput;
mod or;
mod then;

pub use self::maperror::MapError;
pub use self::mapoutput::MapOutput;
pub use self::or::Or;
pub use self::then::Then;
