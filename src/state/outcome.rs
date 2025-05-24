use self::Outcome::*;

/// The non-error outcome of incremental parsing
#[derive(Debug, PartialEq)]
pub enum Outcome<P, O> {
    /// The parser updated its state; a full output has not yet been parsed
    Next(P),

    /// The parser successfully parsed an item
    Parsed(O),
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

/// Extension methods to map outcomes within other structures
pub trait OutcomeExt<P, O> {
    /// The container type produced by mapping the outcome
    type MappedOutcome<P2, O2>;

    /// Map the parser state
    fn map_parser<F, P2>(self, f: F) -> Self::MappedOutcome<P2, O>
    where
        F: FnOnce(P) -> P2;

    /// Map the output
    fn map_output<F, O2>(self, f: F) -> Self::MappedOutcome<P, O2>
    where
        F: FnOnce(O) -> O2;
}

impl<P, O> OutcomeExt<P, O> for Outcome<P, O> {
    type MappedOutcome<P2, O2> = Outcome<P2, O2>;

    /// Map the parser state
    fn map_parser<F, P2>(self, f: F) -> Outcome<P2, O>
    where
        F: FnOnce(P) -> P2,
    {
        match self {
            Next(p) => Next(f(p)),
            Parsed(x) => Parsed(x),
        }
    }

    /// Map the output
    fn map_output<F, O2>(self, f: F) -> Outcome<P, O2>
    where
        F: FnOnce(O) -> O2,
    {
        match self {
            Next(p) => Next(p),
            Parsed(x) => Parsed(f(x)),
        }
    }
}
