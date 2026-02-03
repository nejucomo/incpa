//! [ParserCompose] and supporting types
mod maperror;
mod mapoutput;
mod or;
mod then;

use incpa_state::Input;

use crate::Parser;

pub use self::maperror::MapError;
pub use self::mapoutput::MapOutput;
pub use self::or::Or;
pub use self::then::Then;

/// Every [Parser] also provides this composition interface by blanket impl
pub trait ParserCompose<I: ?Sized + Input>: Parser<I> {
    /// Compose a new parser with mapped output
    fn map_output<F, O>(self, f: F) -> MapOutput<Self, F, O>
    where
        F: FnOnce(Self::Output) -> O,
    {
        MapOutput::new(self, f)
    }

    /// Compose a new parser with mapped error
    fn map_error<F, E>(self, f: F) -> MapError<Self, F, Self::Error>
    where
        F: FnOnce(Self::Error) -> E,
    {
        MapError::new(self, f)
    }

    /// Parse `self` then `other` and return a tuple pair of their outputs on success
    fn then<Q>(self, other: Q) -> Then<Self, Q> {
        Then::new(self, other)
    }

    /// Attempt to parse `self`, and if it fails parse `other`
    fn or<Q>(self, other: Q) -> Or<Self, Q>
    where
        Q: ParserCompose<I, Output = Self::Output, Error = Self::Error>,
    {
        Or::new(self, other)
    }
}

impl<P, I> ParserCompose<I> for P
where
    P: Parser<I>,
    I: ?Sized + Input,
{
}
