#![doc = include_str!("../Description.md")]
#![deny(missing_docs, unsafe_code)]

mod strparser;
pub mod utf8;

pub use self::strparser::StrParser;
