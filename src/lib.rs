#![doc = include_str!("../README.md")]
#![deny(missing_docs, unsafe_code)]

mod buffer;
mod byteparser;
pub mod compose;
mod error;
mod outcome;
mod parser;
pub mod primitive;
mod resultimpls;
mod update;

#[cfg(test)]
pub mod testutils;

pub use self::buffer::Buffer;
pub use self::byteparser::ByteParser;
pub use self::error::Error;
pub use self::outcome::{Outcome, OutcomeExt};
pub use self::parser::Parser;
pub use self::update::{Update, UpdateExt};
