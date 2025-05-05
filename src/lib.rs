#![doc = include_str!("../README.md")]
#![deny(missing_docs, unsafe_code)]

mod combinator;
pub mod combinators;
mod error;
mod literal;
mod output;
mod parser;
pub mod primitive;
pub mod state;

pub use self::combinator::ParserCombinator;
pub use self::error::UniversalParserError;
pub use self::literal::Literal;
pub use self::output::ParserOutput;
pub use self::parser::Parser;
