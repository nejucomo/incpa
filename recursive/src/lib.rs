//! Support parsing recursive grammars with [incpa] parsers
#![deny(missing_docs, unsafe_code)]

mod parser;

pub use self::parser::{InnerParser, RecursiveParser};
