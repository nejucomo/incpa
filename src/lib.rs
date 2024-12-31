//! Incremental parsers which produce either a parsed value or an updated parser state
#![deny(missing_docs, unsafe_code)]

mod buffer;
pub mod compose;
mod error;
mod outcome;
mod parser;
mod result;
mod update;

pub use self::buffer::Buffer;
pub use self::error::Error;
pub use self::outcome::Outcome;
pub use self::parser::Parser;
pub use self::result::{ParseResult, ParseResultExt};
pub use self::update::Update;
