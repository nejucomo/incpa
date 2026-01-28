#![doc = include_str!("../Description.md")]
#![doc = include_str!("../../README-subcrate-link.md")]
#![deny(unsafe_code, missing_docs)]

mod error;
mod impls;
mod input;
mod ioe;

pub use self::error::UniversalParserError;
pub use self::input::Input;
pub use self::ioe::IncpaIOE;
