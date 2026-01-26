#![doc = include_str!("../Description.md")]
#![deny(missing_docs, unsafe_code)]

mod compose;
mod literal;
mod parser;

pub mod combinators;
pub mod primitive;

pub use self::compose::ParserCompose;
pub use self::literal::Literal;
pub use self::parser::Parser;
