use incpa::Parser;

use crate::RecursiveParserState;

/// A parser of a recursive grammar, defined by `P`
#[derive(Debug)]
pub struct RecursiveParser<P>(P);

/// The inner parser of a recursive grammar
#[derive(Debug, Default)]
pub struct RecursionParser {}

impl<P> RecursiveParser<P> {
    /// Construct a recursive grammar parser defined by `P`
    ///
    /// The parser `P` is constructed given an [RecursionParser] which is an opaque parser with the same [Parser::Output] as `P`.
    pub fn new<F>(make_inner: F) -> Self
    where
        F: FnOnce(RecursionParser) -> P,
    {
        RecursiveParser(make_inner(RecursionParser::default()))
    }
}

impl<I, P> Parser<I> for RecursiveParser<P>
where
    I: ?Sized,
    P: Parser<I>,
{
    type Output = P::Output;
    type Error = P::Error;
    type State = RecursiveParserState<P::State>;

    fn start_parser(self) -> Self::State {
        RecursiveParserState::new(self.0.start_parser())
    }
}
