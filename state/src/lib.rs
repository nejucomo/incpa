#![doc = include_str!("../Description.md")]
#![doc = include_str!("../../README-subcrate-link.md")]
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
pub use self::chomped::{Chomped, ChompedResult};
pub use self::error::UniversalParserError;
pub use self::input::Input;
pub use self::outcome::Outcome;
pub use self::state::ParserState;
