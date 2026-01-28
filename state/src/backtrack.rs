use derive_new::new;

use incpa_ioe::{IncpaIOE, Input};

use crate::{Chomped, ChompedResult, Outcome, ParserState};

/// Try to parse `P`, but hold all input until a successful parse
///
/// This ensures if `P` fails with an error, no input will have been consumed. Typically this is used for conditional "look-ahead" parsing.
#[derive(Copy, Clone, Debug, new)]
pub struct Backtrack<P> {
    inner: P,
    #[new(default)]
    consumed: usize,
}

impl<P> IncpaIOE for Backtrack<P>
where
    P: IncpaIOE,
{
    type Input = P::Input;
    type Output = P::Output;
    type Error = P::Error;
}

impl<P> ParserState for Backtrack<P>
where
    P: ParserState,
{
    fn feed(self, input: &Self::Input) -> ChompedResult<Outcome<Self, Self::Output>, Self::Error> {
        use crate::Outcome::{Next, Parsed};

        let inner_input = input.drop_prefix(self.consumed);
        let Chomped {
            consumed,
            value: outcome,
        } = self.inner.feed(inner_input)?;
        let consumed = self.consumed + consumed;

        match outcome {
            Next(inner) => Ok(Chomped {
                consumed: 0,
                value: Next(Backtrack { inner, consumed }),
            }),
            Parsed(output) => Ok(Chomped {
                consumed,
                value: Parsed(output),
            }),
        }
    }

    fn end_input(self, final_input: &Self::Input) -> Result<Self::Output, Self::Error> {
        self.inner.end_input(final_input.drop_prefix(self.consumed))
    }
}
