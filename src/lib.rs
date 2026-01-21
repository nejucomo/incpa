#![doc = include_str!("../README.md")]
#![deny(missing_docs, unsafe_code)]

pub mod combinators;
mod compose;
mod error;
mod input;
mod literal;
mod outerr;
mod parser;
pub mod primitive;
pub mod state;

pub use self::compose::ParserCompose;
pub use self::error::UniversalParserError;
pub use self::input::Input;
pub use self::literal::Literal;
pub use self::outerr::ParserOutErr;
pub use self::parser::Parser;
