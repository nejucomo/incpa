//! [ByteParser] and other support for parsing bytes
//!
//! <details>
//! <summary>TODO</summary>
//!
//! - Replace our concrete `BufferManager` with a trait and impl, because different applications have different performance needs and benefit from different strategies.
//! </details>
#![deny(missing_docs, unsafe_code)]

mod bufmgr;
mod parser;
pub mod testutils;

pub use self::bufmgr::BufferManager;
pub use self::parser::ByteParser;
