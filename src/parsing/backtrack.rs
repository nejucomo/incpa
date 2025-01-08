use derive_new::new;

use crate::parsing::{Buffer, Parser, Update};
use crate::BaseParserError;

/// Try to parse `P`, but hold all input until a successful parse
///
/// This ensures if `P` fails with an error, no input will have been consumed. Typically this is used internally for conditional parsing, such as with [syntax::Or](crate::syntax::Or).
#[derive(Copy, Clone, Debug, new)]
pub struct Backtrack<P> {
    inner: P,
    #[new(default)]
    consumed: usize,
}

// impl<P, I, O, E> Syntax<I, O, E> for Backtrack<P> where P: Syntax<I, O, E> {}

impl<P, I, O, E> Parser<I, O, E> for Backtrack<P>
where
    I: ?Sized + Buffer,
    P: Parser<I, O, E>,
    E: From<BaseParserError>,
{
    fn feed(self, input: &I) -> Result<Update<Self, O>, E> {
        use crate::parsing::Outcome::{Next, Parsed};

        let inner_input = input.drop_prefix(self.consumed);
        let Update { consumed, outcome } = self.inner.feed(inner_input)?;
        let consumed = self.consumed + consumed;

        match outcome {
            Next(inner) => Ok(Update {
                consumed: 0,
                outcome: Next(Backtrack { inner, consumed }),
            }),
            Parsed(output) => Ok(Update {
                consumed,
                outcome: Parsed(output),
            }),
        }
    }

    fn end_input(self, final_input: &I) -> Result<O, E> {
        self.inner.end_input(final_input.drop_prefix(self.consumed))
    }
}
