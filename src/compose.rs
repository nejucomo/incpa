//! Utilities for composing parsers
//!
//! Typically these utilities are produced by using methods of [Parser](crate::Parser) directly.

mod maperror;
mod mapoutput;

pub use self::maperror::MapError;
pub use self::mapoutput::MapOutput;
