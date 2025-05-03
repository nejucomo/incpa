//! Support parsing recursive grammars with [incpa] parsers
#![deny(missing_docs, unsafe_code)]

mod parser;
mod state;
mod status;

pub use crate::parser::RecursiveParser;
pub use crate::state::{FeedChompedRecursionStatus, RecursiveParserState};
pub use crate::status::RecursionStatus;
