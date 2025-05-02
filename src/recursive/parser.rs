use crate::Parser;
use crate::recursive::{Recursion, RecursiveInnerState, RecursiveParserState};

/// A parser which internally parses a [Recursion] of itself
#[derive(Debug)]
pub struct RecursiveParser<P>(P);

impl<P> RecursiveParser<P> {
    /// Construct a new recursive parser from a closure which creates an inner parser given a recursion
    pub fn new<F>(parser_from: F) -> Self
    where
        F: FnOnce(Recursion) -> P,
    {
        RecursiveParser(parser_from(Recursion))
    }
}

impl<I, P> Parser<I> for RecursiveParser<P>
where
    I: ?Sized,
    P: Parser<I>,
    P::State: RecursiveInnerState<I>,
{
    type Output = P::Output;
    type Error = P::Error;
    type State = RecursiveParserState<I, P::State>;

    fn start_parser(self) -> Self::State {
        RecursiveParserState::new(self.0.start_parser())
    }
}
