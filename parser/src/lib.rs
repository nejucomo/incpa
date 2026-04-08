#![doc = include_str!("../Description.md")]
#![doc = include_str!("../../README-subcrate-link.md")]
#![deny(missing_docs, unsafe_code)]

mod compose;
mod literal;
mod parser;
pub mod primitive;
/// Recursive parser combinator; see [recursive::recursive] for usage
pub mod recursive;

pub use self::compose::ParserCompose;
pub use self::literal::Literal;
pub use self::parser::Parser;
pub use self::recursive::{recursive, Recursive};
