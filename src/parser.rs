use std::future::Future;

use crate::compose::{MapError, MapOutput};
use crate::Error::{self, ExpectedMoreInput, UnexpectedInput};
use crate::{Buffer, Outcome, Update};

/// The essential incremental parser trait parses references to input `I` to produce an output `O` or an error `E`
pub trait Parser<I, O, E = Error>: Sized
where
    I: ?Sized,
{
    /// Feed an input reference to the parser to produce an update
    fn feed(self, input: &I) -> Result<Update<Self, O>, E>;

    /// Unwrap a pending output
    ///
    /// Consumer code typically calls `self.end_input()` instead of this implementor method.
    fn unwrap_pending(self) -> Option<O>;

    /// Parse an entire in-memory input completely
    fn parse(self, complete_input: &I) -> Result<O, E>
    where
        I: Buffer,
        E: From<Error>,
    {
        use crate::Outcome::{Next, Parsed};

        let Update { consumed, outcome } = self.feed(complete_input)?;
        if consumed == complete_input.len() {
            match outcome {
                Next(p) => p.end_input(),
                Parsed(output) => Ok(output),
            }
        } else {
            Err(E::from(UnexpectedInput))
        }
    }

    /// Inform the parser there is no more input; it either produces a pending value or expects more input
    fn end_input(self) -> Result<O, E>
    where
        E: From<Error>,
    {
        self.unwrap_pending().ok_or(E::from(ExpectedMoreInput))
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
}
