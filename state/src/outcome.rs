use crate::map::{MapNext, MapParsed};

use self::Outcome::*;

/// The non-error outcome of incremental parsing
#[derive(Debug, PartialEq)]
pub enum Outcome<S, P> {
    /// The parser updated its state; a full output has not yet been parsed
    Next(S),

    /// The parser successfully parsed an item
    Parsed(P),
}

impl<S, O, E> Outcome<S, Result<O, E>> {
    /// Convert an [Outcome] with a [Result] output to a [Result] containing [Outcome]
    ///
    /// This may be useful after [Outcome::map_parsed] if mapped to a [Result].
    pub fn transpose_output(self) -> Result<Outcome<S, O>, E> {
        match self {
            Next(s) => Ok(Next(s)),
            Parsed(Ok(x)) => Ok(Parsed(x)),
            Parsed(Err(e)) => Err(e),
        }
    }
}

impl<S, P> MapNext<S> for Outcome<S, P> {
    type MappedNext<T> = Outcome<T, P>;

    fn map_next<F, U>(self, f: F) -> Self::MappedNext<U>
    where
        F: FnOnce(S) -> U,
    {
        match self {
            Next(s) => Next(f(s)),
            Parsed(p) => Parsed(p),
        }
    }
}

impl<S, P> MapParsed<P> for Outcome<S, P> {
    type MappedParsed<Q> = Outcome<S, Q>;

    fn map_parsed<F, U>(self, f: F) -> Self::MappedParsed<U>
    where
        F: FnOnce(P) -> U,
    {
        match self {
            Next(s) => Next(s),
            Parsed(p) => Parsed(f(p)),
        }
    }
}
