/// The non-error outcome of incremental parsing
#[derive(Debug, PartialEq)]
pub enum Outcome<P, O> {
    /// The parser updated its state; a full output has not yet been parsed
    Next(P),

    /// The parser successfully parsed an item
    Parsed(O),
}
use Outcome::*;

impl<P, O> Outcome<P, O> {
    /// Map the parser state
    pub fn map_parser<F, P2>(self, f: F) -> Outcome<P2, O>
    where
        F: FnOnce(P) -> P2,
    {
        match self {
            Next(p) => Next(f(p)),
            Parsed(x) => Parsed(x),
        }
    }

    /// Map the output
    pub fn map_output<F, O2>(self, f: F) -> Outcome<P, O2>
    where
        F: FnOnce(O) -> O2,
    {
        match self {
            Next(p) => Next(p),
            Parsed(x) => Parsed(f(x)),
        }
    }
}

impl<P, O, E> Outcome<P, Result<O, E>> {
    /// Convert an [Outcome] with a [Result] output to a [Result] containing [Outcome]
    ///
    /// This may be useful after [Outcome::map_output] if mapped to a [Result].
    pub fn transpose_output(self) -> Result<Outcome<P, O>, E> {
        match self {
            Next(s) => Ok(Next(s)),
            Parsed(Ok(x)) => Ok(Parsed(x)),
            Parsed(Err(e)) => Err(e),
        }
    }
}
