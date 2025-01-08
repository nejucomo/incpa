//! [Syntax] and impls for composing syntaxes to specify a parser's behavior
mod byteformat;
mod literal;
mod maperror;
mod mapoutput;
mod or;
mod then;

pub use self::byteformat::ByteFormat;
pub use self::literal::{Literal, LiteralParser};
pub use self::maperror::MapError;
pub use self::mapoutput::MapOutput;
pub use self::or::Or;
pub use self::then::Then;

use crate::parsing::{Buffer, Parser};
use crate::BaseParserError;

/// A [Syntax] defines the syntax, grammar, or format to be parsed
///
/// Implementations can often specify the grammar to be parsed by [crate::primitive] types and the composition methods of this trait.
///
/// The actual behind-the-scenes work of parsing is accomplished by creating [Syntax::State] from [Syntax::into_parser], then driving that.
pub trait Syntax<I, O, E = BaseParserError>: Sized
where
    I: ?Sized,
    E: From<BaseParserError>,
{
    /// The initial [Parser] to parse this specification
    type State: Parser<I, O, E>;

    /// Construct a state to drive low-level parsing
    fn into_parser(self) -> Self::State;

    /// Parse an entire in-memory input to completion
    fn parse_all(self, input: &I) -> Result<O, E>
    where
        I: Buffer,
    {
        use crate::parsing::Outcome::{Next, Parsed};
        use crate::parsing::Update;

        let Update { consumed, outcome } = self.into_parser().feed(input)?;
        match outcome {
            Next(p) => p.end_input(input.drop_prefix(consumed)),
            Parsed(output) => Ok(output),
        }
    }

    /// Compose a new parser with mapped output
    fn map<F, O2>(self, f: F) -> MapOutput<Self, F, O>
    where
        F: FnOnce(O) -> O2,
    {
        MapOutput::new(self, f)
    }

    /// Compose a new parser with mapped error
    fn map_error<F, E2>(self, f: F) -> MapError<Self, F, E>
    where
        F: FnOnce(E) -> E2,
    {
        MapError::new(self, f)
    }

    /// Parse `self` then `other` and return a tuple pair of their outputs on success
    fn then<Q>(self, other: Q) -> Then<Self, O, Q> {
        Then::new(self, other)
    }

    /// Attempt to parse `self`, and if it fails parse `other`
    fn or<Q>(self, other: Q) -> Or<Self, Q> {
        Or::new(self, other)
    }
}
