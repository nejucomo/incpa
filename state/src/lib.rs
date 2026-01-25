//! [ParserState] and abstractions to support it
//!
//! This is the heart of `incpa`, a suite of crates for constructing or consuming incremental parser. See the `incpa` crate docs for an intro. Typically only lower-level parser implementations need to be concerned with this crate directly.

#![deny(unsafe_code, missing_docs)]

mod backtrack;
mod chomped;
mod error;
mod input;
mod outcome;
mod resultimpls;
mod state;

pub mod map;

pub use self::backtrack::Backtrack;
pub use self::chomped::{Chomped, FeedChomped};
pub use self::error::UniversalParserError;
pub use self::input::Input;
pub use self::outcome::Outcome;
pub use self::state::ParserState;
