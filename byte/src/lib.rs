#![doc = include_str!("../Description.md")]
#![doc = include_str!("../../README-subcrate-link.md")]
//! # TODO
//!
//! - Replace our concrete `BufferManager` with a trait and impl, because different applications have different performance needs and benefit from different strategies.
#![deny(missing_docs, unsafe_code)]

mod bufmgr;
mod parser;
pub mod testutils;

pub use self::bufmgr::BufferManager;
pub use self::parser::ByteParser;
