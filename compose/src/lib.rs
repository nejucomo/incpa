#![doc = include_str!("../Description.md")]
#![doc = include_str!("../../README-subcrate-link.md")]
#![deny(unsafe_code, missing_docs)]

mod compose;
mod eitheror;
mod maperror;
mod mapoutput;
mod or;
mod then;

pub use self::compose::ParserCompose;
pub use self::eitheror::EitherOr;
pub use self::maperror::MapError;
pub use self::mapoutput::MapOutput;
pub use self::or::Or;
pub use self::then::Then;
