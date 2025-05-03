/// A parser of a recursive grammar, defined by `P`
#[derive(Debug)]
pub struct RecursiveParser<P>(P);

/// The inner parser of a recursive grammar
#[derive(Debug, Default)]
pub struct InnerParser {}

impl<P> RecursiveParser<P> {
    /// Construct a recursive grammar parser defined by `P`
    ///
    /// The parser `P` is constructed given an [InnerParser] which is an opaque parser with the same [Parser::Output] as `P`.
    pub fn new<F>(make_inner: F) -> Self
    where
        F: FnOnce(InnerParser) -> P,
    {
        RecursiveParser(make_inner(InnerParser::default()))
    }
}
