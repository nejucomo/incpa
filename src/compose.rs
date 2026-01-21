use crate::UniversalParserError;
use crate::combinators::{MapError, MapOutput, Or, Then};

/// A base trait for parser which enable high-level composition methods
pub trait ParserCompose: Sized {
    /// The type of output on successful parse
    type Output;

    /// The type of errors this parser detects
    type Error: From<UniversalParserError>;

    /// Compose a new parser with mapped output
    fn map<F, O>(self, f: F) -> MapOutput<Self, F, O>
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
    fn then<Q>(self, other: Q) -> Then<Self, Q>
    where
        Q: ParserCompose<Error = Self::Error>,
    {
        Then::new(self, other)
    }

    /// Parse `self` then `other` and return only `self`'s output
    fn then_ignore<Q>(self, other: Q) -> impl ParserCompose<Output = Self::Output>
    where
        Q: ParserCompose<Error = Self::Error>,
    {
        self.then(other).map(|(val, _)| val)
    }

    /// Attempt to parse `self`, and if it fails parse `other`
    fn or<Q>(self, other: Q) -> Or<Self, Q> {
        Or::new(self, other)
    }
}
