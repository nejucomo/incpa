/// The result of incremental parsing (when an error is not encountered)
#[derive(Debug, PartialEq)]
pub enum Outcome<S, X> {
    /// The parser updated its state; a full output has not yet been parsed
    Next(S),

    /// The parser successfully parsed an item
    Parsed(X),
}
use Outcome::*;

impl<S, X> Outcome<S, X> {
    /// Map the pending state
    pub fn map_next<F, S2>(self, f: F) -> Outcome<S2, X>
    where
        F: FnOnce(S) -> S2,
    {
        match self {
            Next(s) => Next(f(s)),
            Parsed(x) => Parsed(x),
        }
    }

    /// Map the output
    pub fn map_output<F, X2>(self, f: F) -> Outcome<S, X2>
    where
        F: FnOnce(X) -> X2,
    {
        match self {
            Next(s) => Next(s),
            Parsed(x) => Parsed(f(x)),
        }
    }
}

impl<S, X, E> Outcome<S, Result<X, E>> {
    /// Convert an [Outcome] with a [Result] output to a [Result] containing [Outcome]
    ///
    /// This may be useful after [Outcome::map_output] if mapped to a [Result].
    pub fn transpose_output(self) -> Result<Outcome<S, X>, E> {
        match self {
            Next(s) => Ok(Next(s)),
            Parsed(Ok(x)) => Ok(Parsed(x)),
            Parsed(Err(e)) => Err(e),
        }
    }
}
