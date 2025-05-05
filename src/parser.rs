use crate::ParserCombinator;
use crate::state::{Buffer, ParserState};

/// A [Parser] defines the syntax, grammar, or format to be parsed
///
/// Implementations can often specify the grammar to be parsed by [crate::primitive] types and the composition methods of this trait.
///
/// The actual behind-the-scenes work of parsing is accomplished by creating [Parser::State] from [Parser::start_parser], then driving that.
pub trait Parser<I>: ParserCombinator
where
    I: ?Sized,
{
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
}
