//! Incremental parsers which produce either a parsed value or an updated parser state
#![deny(missing_docs, unsafe_code)]
mod outcome;
mod parser;
mod update;

pub use self::outcome::Outcome;
pub use self::parser::Parser;
pub use self::update::Update;
