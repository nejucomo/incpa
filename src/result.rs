use crate::{Outcome, Update};

/// The incremental parse result of [Parser::feed](crate::Parser::feed)
pub type ParseResult<P, O, E> = Result<Update<P, O>, E>;

/// An extension trait to simplify modifying [ParseResult]
pub trait ParseResultExt<P, O, E> {
    /// Map the outcome
    fn map_outcome<F, P2, O2>(self, f: F) -> ParseResult<P2, O2, E>
    where
        F: FnOnce(Outcome<P, O>) -> Outcome<P2, O2>;

    /// Map the pending state
    fn map_parser<F, P2>(self, f: F) -> ParseResult<P2, O, E>
    where
        F: FnOnce(P) -> P2;

    /// Map the output
    fn map_output<F, O2>(self, f: F) -> ParseResult<P, O2, E>
    where
        F: FnOnce(O) -> O2;
}

impl<P, O, E> ParseResultExt<P, O, E> for ParseResult<P, O, E> {
    fn map_outcome<F, P2, O2>(self, f: F) -> ParseResult<P2, O2, E>
    where
        F: FnOnce(Outcome<P, O>) -> Outcome<P2, O2>,
    {
        self.map(|up| up.map_outcome(f))
    }

    fn map_parser<F, P2>(self, f: F) -> ParseResult<P2, O, E>
    where
        F: FnOnce(P) -> P2,
    {
        self.map(|up| up.map_parser(f))
    }

    fn map_output<F, O2>(self, f: F) -> ParseResult<P, O2, E>
    where
        F: FnOnce(O) -> O2,
    {
        self.map(|up| up.map_output(f))
    }
}
