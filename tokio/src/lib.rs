//! support async streaming input via [tokio::io::AsyncRead] in [incpa] parsers
#![deny(missing_docs, unsafe_code)]
mod byteparserext;

pub use self::byteparserext::ByteParserExt;
