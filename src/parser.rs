use crate::UniversalParserError;
use crate::combinators::{MapError, MapOutput, Or, Then};
use crate::state::{Buffer, ParserState};

/// A [Parser] defines the syntax, grammar, or format to be parsed
///
/// Implementations can often specify the grammar to be parsed by [crate::primitive] types and the composition methods of this trait.
///
/// The actual behind-the-scenes work of parsing is accomplished by creating [Parser::State] from [Parser::start_parser], then driving that.
pub trait Parser<I>: Sized
where
    I: ?Sized,
{
    /// The type of output on successful parse
    type Output;

    /// The type of errors this parser detects
    type Error: From<UniversalParserError>;

    /// The initial [ParserState] to parse this specification
    type State: ParserState<I, Output = Self::Output, Error = Self::Error>;

    /// Construct a state to drive low-level parsing
    fn start_parser(self) -> Self::State;

    /// Parse an entire in-memory input to completion
    fn parse_all(self, input: &I) -> Result<Self::Output, Self::Error>
    where
        I: Buffer,
    {
        use crate::state::Chomped;
        use crate::state::Outcome::{Next, Parsed};

        let Chomped { consumed, value } = self.start_parser().feed(input)?;
        match value {
            Next(p) => p.end_input(input.drop_prefix(consumed)),
            Parsed(output) => Ok(output),
        }
    }

    /// Compose a new parser with mapped output
    fn map<F, O>(self, f: F) -> MapOutput<I, Self, F, O>
    where
        F: FnOnce(Self::Output) -> O,
    {
        MapOutput::new(self, f)
    }

    /// Compose a new parser with mapped error
    fn map_error<F, E>(self, f: F) -> MapError<I, Self, F, Self::Error>
    where
        F: FnOnce(Self::Error) -> E,
    {
        MapError::new(self, f)
    }

    /// Parse `self` then `other` and return a tuple pair of their outputs on success
    fn then<Q>(self, other: Q) -> Then<I, Self, Q>
    where
        I: Buffer,
        Q: Parser<I, Error = Self::Error>,
    {
        Then::new(self, other)
    }

    /// Attempt to parse `self`, and if it fails parse `other`
    fn or<Q>(self, other: Q) -> Or<I, Self, Q>
    where
        I: Buffer,
        Q: Parser<I, Error = Self::Error>,
    {
        Or::new(self, other)
    }
}
