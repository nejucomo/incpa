#![doc = include_str!("../README.md")]
#![deny(missing_docs, unsafe_code)]

mod error;
pub mod parsing;
pub mod primitive;
pub mod syntax;
pub mod testutils;

pub use self::error::BaseParserError;
pub use self::syntax::Parser;
