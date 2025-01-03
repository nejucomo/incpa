#[cfg(test)]
mod tests;

mod arrayimpl;
mod charimpl;
mod sliceimpl;
mod strimpl;

use derive_new::new;

use crate::parsing::{Buffer, Parser, Update};
use crate::{BaseParserError, Syntax};

/// A `Literal` is any value which is syntax for a parser of itself
pub trait Literal<I, E>: Sized + Copy + Syntax<I, Self, E>
where
    I: ?Sized + Buffer,
    E: From<BaseParserError>,
{
    /// The length of this literal in `I`'s units
    fn literal_len(self) -> usize;

    /// Is this literal equal to the candidate
    ///
    /// # Preconditions
    ///
    /// - `candidate.len() == self.literal_len()`
    fn literal_eq(self, candidate: &I) -> bool;
}

/// Parse a literal value
#[derive(Copy, Clone, Debug, new)]
pub struct LiteralParser<L>(L);

impl<I, L, E> Parser<I, L, E> for LiteralParser<L>
where
    I: ?Sized + Buffer,
    L: Literal<I, E>,
    E: From<BaseParserError>,
{
    fn feed(self, input: &I) -> Result<Update<Self, L>, E> {
        use crate::parsing::Outcome::{Next, Parsed};
        use crate::BaseParserError::UnexpectedInput;

        let n = self.0.literal_len();
        let prefix = input.prefix_up_to(n);

        if prefix.len() < n {
            Ok(Update::new(0, Next(self)))
        } else if self.0.literal_eq(prefix) {
            Ok(Update::new(n, Parsed(self.0)))
        } else {
            Err(E::from(UnexpectedInput))
        }
    }
}