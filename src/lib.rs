#![doc = include_str!("../README.md")]
#![deny(missing_docs, unsafe_code)]

pub mod compose;
mod error;
pub mod parsing;
pub mod primitive;
mod syntax;

#[cfg(test)]
pub mod testutils;

pub use self::error::BaseParserError;
pub use self::syntax::Syntax;
