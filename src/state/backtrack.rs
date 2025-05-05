use derive_new::new;

use crate::ParserOutput;
use crate::state::{Buffer, Chomped, FeedChomped, ParserState};

/// Try to parse `P`, but hold all input until a successful parse
///
/// This ensures if `P` fails with an error, no input will have been consumed. Typically this is used internally for conditional parsing, such as with [combinators::Or](crate::combinators::Or).
#[derive(Copy, Clone, Debug, new)]
pub struct Backtrack<P> {
    inner: P,
    #[new(default)]
    consumed: usize,
}

impl<P> ParserOutput for Backtrack<P>
where
    P: ParserOutput,
{
    type Output = P::Output;
    type Error = P::Error;
}

impl<P, I> ParserState<I> for Backtrack<P>
where
    I: ?Sized + Buffer,
    P: ParserState<I>,
{
    fn feed(self, input: &I) -> Result<FeedChomped<Self, Self::Output>, Self::Error> {
        use crate::state::Outcome::{Next, Parsed};

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

    fn end_input(self, final_input: &I) -> Result<Self::Output, Self::Error> {
        self.inner.end_input(final_input.drop_prefix(self.consumed))
    }
}
