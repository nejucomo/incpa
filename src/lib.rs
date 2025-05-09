#![doc = include_str!("../README.md")]
#![deny(missing_docs, unsafe_code)]

pub mod combinators;
mod error;
mod literal;
mod parser;
pub mod primitive;
pub mod recursive;
pub mod state;

pub use self::error::UniversalParserError;
pub use self::literal::Literal;
pub use self::parser::Parser;
