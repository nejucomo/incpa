//! [tokio::io::AsyncRead] support via [ByteParserExt] makes parsing async sources concise
#![deny(missing_docs, unsafe_code)]
mod byteparserext;

pub use self::byteparserext::ByteParserExt;
