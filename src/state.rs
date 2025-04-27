//! [ParserState] and abstractions to support it
mod backtrack;
mod buffer;
mod chomped;
mod outcome;
mod resultimpls;

pub use self::backtrack::Backtrack;
pub use self::buffer::Buffer;
pub use self::chomped::{Chomped, ChompedExt, FeedChomped};
pub use self::outcome::{Outcome, OutcomeExt};

// ParserState below
use std::future::Future;

use crate::BaseParserError::{self, ExpectedMoreInput};

/// A [ParserState] represents in-progress parsing
///
/// # Invariants
///
/// This crate assumes every [ParserState] impl is deterministic, so that calling [ParserState::feed] or [ParserState::end_input] on two equivalent states with the same input parameters produces equivalent values.
pub trait ParserState<I>: Sized
where
    I: ?Sized,
{
    /// The type of output on successful parse
    type Output;

    /// The type of errors this parser detects
    type Error: From<BaseParserError>;

    /// Feed an input reference to the parser to produce an update
    ///
    /// Precondition: `input` includes a suffix which has not been seen previously by this parser.
    fn feed(self, input: &I) -> Result<FeedChomped<Self, Self::Output>, Self::Error>;

    /// Inform the parser there is no more input; it either produces a pending value or expects more input
    ///
    /// The default implementation simply returns the [ExpectedMoreInput] error.
    ///
    /// Precondition: all of `final_input` must have been seen by a prior call to [ParserState::feed]
    fn end_input(self, final_input: &I) -> Result<Self::Output, Self::Error> {
        let _ = final_input;
        Err(Self::Error::from(ExpectedMoreInput))
    }

    /// Repeatedly update a parser in a loop until it produces an error or value
    fn run_parser<F, E>(self, mut f: F) -> Result<Self::Output, E>
    where
        F: FnMut(Self) -> Result<Outcome<Self, Self::Output>, E>,
    {
        use Outcome::{Next, Parsed};

        let mut parser = self;
        loop {
            match f(parser)? {
                Next(p) => {
                    parser = p;
                }

                Parsed(output) => {
                    return Ok(output);
                }
            }
        }
    }

    /// Repeatedly update a parser in an async loop until it produces an error or value
    fn run_parser_async<F, E, S, Fut>(
        self,
        init: S,
        f: F,
    ) -> impl Future<Output = Result<Self::Output, E>>
    where
        F: Fn(Self, S) -> Fut,
        Fut: Future<Output = Result<Outcome<(Self, S), Self::Output>, E>>,
    {
        use Outcome::{Next, Parsed};

        async move {
            let mut parser = self;
            let mut state = init;
            loop {
                match f(parser, state).await? {
                    Next((p, st)) => {
                        parser = p;
                        state = st;
                    }

                    Parsed(output) => {
                        return Ok(output);
                    }
                }
            }
        }
    }
}
