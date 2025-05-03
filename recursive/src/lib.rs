//! Support parsing recursive grammars with [incpa] parsers
#![deny(missing_docs, unsafe_code)]

mod parser;
mod state;

pub use self::parser::{RecursionParser, RecursiveParser};
pub use self::state::RecursiveParserState;
