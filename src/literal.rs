#[cfg(test)]
mod tests;

mod arrayimpl;
mod charimpl;
mod sliceimpl;
mod strimpl;

use derive_new::new;

use crate::Parser;
use crate::state::{Buffer, FeedUpdate, ParserState, Update};

/// A [Literal] is any value which is a [Parser] for itself
///
/// # Example
///
/// ```
/// use incpa::BaseParserError;
/// use incpa::{Parser, Literal};
///
/// fn main() -> Result<(), BaseParserError> {
///   // &str is a Literal, so it can parse an input:
///   let literal = "Hello World!";
///   let parsed = literal.parse_all("Hello World!")?;
///   // A literal parses itself as input to produce itself as output:
///   assert_eq!(parsed, "Hello World!");
///   Ok(())
/// }
/// ```
pub trait Literal<I>: Sized + Copy + Parser<I, Output = Self>
where
    I: ?Sized + Buffer,
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

impl<I, L> ParserState<I> for LiteralParser<L>
where
    I: ?Sized + Buffer,
    L: Literal<I>,
{
    type Output = L::Output;
    type Error = L::Error;

    fn feed(self, input: &I) -> Result<FeedUpdate<Self, L>, Self::Error> {
        use crate::BaseParserError::UnexpectedInput;
        use crate::state::Outcome::{Next, Parsed};

        let n = self.0.literal_len();
        let prefix = input.prefix_up_to(n);

        if prefix.len() < n {
            Ok(Update::new(0, Next(self)))
        } else if self.0.literal_eq(prefix) {
            Ok(Update::new(n, Parsed(self.0)))
        } else {
            Err(Self::Error::from(UnexpectedInput))
        }
    }
}
