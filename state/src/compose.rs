//! [ParserState](crate::ParserState) impls for the [ParserCompose](incpa_compose::ParserCompose) compositions
mod eitheror;
mod maperror;
mod mapoutput;
mod or;
mod then;

pub use self::eitheror::EitherOrState;
pub use self::or::OrState;
pub use self::then::ThenState;
