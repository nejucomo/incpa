#![doc = include_str!("../README.md")]
#![deny(missing_docs, unsafe_code)]

pub mod combinators;
mod error;
mod literal;
mod parameterized;
mod parser;
pub mod primitive;
pub mod state;

pub use self::error::BaseParserError;
pub use self::literal::Literal;
pub use self::parameterized::ParameterizedParser;
pub use self::parser::Parser;
