#![doc = include_str!("../../README.md")]
#![deny(missing_docs, unsafe_code)]

mod byteparser;
pub mod combinators;
mod error;
mod literal;
mod parser;
pub mod parsing;
pub mod primitive;
pub mod testutils;

pub use self::byteparser::ByteParser;
pub use self::error::BaseParserError;
pub use self::literal::Literal;
pub use self::parser::Parser;
