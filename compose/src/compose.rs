use incpa_ioe::IncpaIOE;

use crate::{MapError, MapOutput, Or, Then};

/// A base-trait of parsers providing composition
pub trait ParserCompose: IncpaIOE {
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
    fn or<Q>(self, other: Q) -> Or<Self, Q> {
        Or::new(self, other)
    }
}

// `std` foreign impls
impl ParserCompose for char {}

impl ParserCompose for &str {}

impl<T> ParserCompose for &[T] where T: PartialEq {}

impl<T, const K: usize> ParserCompose for &[T; K] where T: PartialEq {}
