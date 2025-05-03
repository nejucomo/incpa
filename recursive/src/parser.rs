use incpa::UniversalParserError;

use crate::RecursiveParserState;

/// A parser for a recursive grammar
///
/// Parsing a recursion produces an `R` value, which is distinct from [Self::Output]. However, when `R = Self::Output` we can adapt to the base [incpa::Parser]
pub trait RecursiveParser<I, R>: Sized
where
    I: ?Sized,
{
    /// The type of output on successful parse
    type Output;

    /// The type of errors this parser detects
    type Error: From<UniversalParserError>;

    /// The initial [RecursiveParserState] to parse this specification
    type State: RecursiveParserState<I, R, Output = Self::Output, Error = Self::Error>;
}
