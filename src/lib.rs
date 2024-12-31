#![doc = include_str!("../README.md")]
#![deny(missing_docs, unsafe_code)]

mod buffer;
mod byteparser;
pub mod compose;
mod error;
mod outcome;
mod parser;
mod result;
mod update;

pub use self::buffer::Buffer;
pub use self::byteparser::ByteParser;
pub use self::error::Error;
pub use self::outcome::Outcome;
pub use self::parser::Parser;
pub use self::result::{ParseResult, ParseResultExt};
pub use self::update::Update;
