use std::future::Future;

use crate::parsing::{Outcome, Update};
use crate::BaseParserError::{self, ExpectedMoreInput};

/// A [Parser] represents in-progress parsing
///
/// # Invariants
///
/// This crate assumes every [Parser] impl is deterministic, so that calling [Parser::feed] or [Parser::end_input] on two equivalent states with the same input parameters produces equivalent values.
pub trait Parser<I, O, E>: Sized
where
    I: ?Sized,
    E: From<BaseParserError>,
{
    /// Feed an input reference to the parser to produce an update
    ///
    /// Precondition: `input` includes a suffix which has not been seen previously by this parser.
    fn feed(self, input: &I) -> Result<Update<Self, O>, E>;

    /// Inform the parser there is no more input; it either produces a pending value or expects more input
    ///
    /// The default implementation simply returns the [ExpectedMoreInput] error.
    ///
    /// Precondition: all of `final_input` must have been seen by a prior call to [Parser::feed]
    fn end_input(self, final_input: &I) -> Result<O, E> {
        let _ = final_input;
        Err(E::from(ExpectedMoreInput))
    }

    /// Repeatedly update a parser in a loop until it produces an error or value
    fn run_parser<F>(self, mut f: F) -> Result<O, E>
    where
        F: FnMut(Self) -> Result<Outcome<Self, O>, E>,
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
    fn run_parser_async<F, S, Fut>(self, init: S, f: F) -> impl Future<Output = Result<O, E>>
    where
        F: Fn(Self, S) -> Fut,
        Fut: Future<Output = Result<Outcome<(Self, S), O>, E>>,
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
