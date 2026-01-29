#![doc = include_str!("../Description.md")]
#![doc = include_str!("../../README-subcrate-link.md")]
#![deny(missing_docs, unsafe_code)]

mod compimpls;
mod literal;
mod parser;

pub mod primitive;

pub use self::literal::Literal;
pub use self::parser::Parser;
