//! Support parsing recursive grammars with [incpa] parsers
#![deny(missing_docs, unsafe_code)]

mod parser;
mod recursion;

pub use self::parser::recursive_parser;
pub use self::recursion::Recursion;
