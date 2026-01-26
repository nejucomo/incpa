#![doc = include_str!("../../README.md")]
#![deny(missing_docs, unsafe_code)]

pub use incpa_parser as parser;
pub use incpa_parser::Parser;

pub use incpa_state as state;

pub use incpa_byte::ByteParser;
pub use incpa_str::{StrParser, utf8};

pub use incpa_tokio as tokio;
