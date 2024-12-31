/// The non-error outcome of incremental parsing
#[derive(Debug, PartialEq)]
pub enum Outcome<S, O> {
    /// The parser updated its state; a full output has not yet been parsed
    Next(S),

    /// The parser successfully parsed an item
    Parsed(O),
}
use Outcome::*;

impl<S, O> Outcome<S, O> {
    /// Map the pending state
    pub fn map_next<F, S2>(self, f: F) -> Outcome<S2, O>
    where
        F: FnOnce(S) -> S2,
    {
        match self {
            Next(s) => Next(f(s)),
            Parsed(x) => Parsed(x),
        }
    }

    /// Map the output
    pub fn map_output<F, O2>(self, f: F) -> Outcome<S, O2>
    where
        F: FnOnce(O) -> O2,
    {
        match self {
            Next(s) => Next(s),
            Parsed(x) => Parsed(f(x)),
        }
    }
}

impl<S, O, E> Outcome<S, Result<O, E>> {
    /// Convert an [Outcome] with a [Result] output to a [Result] containing [Outcome]
    ///
    /// This may be useful after [Outcome::map_output] if mapped to a [Result].
    pub fn transpose_output(self) -> Result<Outcome<S, O>, E> {
        match self {
            Next(s) => Ok(Next(s)),
            Parsed(Ok(x)) => Ok(Parsed(x)),
            Parsed(Err(e)) => Err(e),
        }
    }
}
