//! [ByteParser] and other support for parsing bytes
//!
//! <details>
//! <summary>TODO</summary>
//!
//! - Replace our concrete `BufferManager` with a trait and impl, because different applications have different performance needs and benefit from different strategies.
//! </details>
mod bufmgr;
mod parser;

pub use self::bufmgr::BufferManager;
pub use self::parser::ByteParser;
