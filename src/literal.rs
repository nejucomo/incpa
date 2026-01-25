#[cfg(test)]
mod tests;

mod arrayimpl;
mod charimpl;
mod sliceimpl;
mod strimpl;

use derive_new::new;

use crate::state::{Chomped, FeedChomped, ParserState};
use crate::{Input, Parser};

/// A [Literal] is any value which is a [Parser] for itself
///
/// # Example
///
/// ```
/// use incpa::UniversalParserError;
/// use incpa::{Parser, Literal};
///
/// fn main() -> Result<(), UniversalParserError> {
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
    I: ?Sized + Input,
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
    I: ?Sized + Input,
    L: Literal<I>,
{
    type Output = L::Output;
    type Error = L::Error;

    fn feed(self, input: &I) -> Result<FeedChomped<Self, L>, Self::Error> {
        use crate::UniversalParserError::UnexpectedInput;
        use crate::state::Outcome::{Next, Parsed};

        let n = self.0.literal_len();
        let prefix = input.prefix_up_to(n);

        if prefix.len() < n {
            Ok(Chomped::new(0, Next(self)))
        } else if self.0.literal_eq(prefix) {
            Ok(Chomped::new(n, Parsed(self.0)))
        } else {
            Err(Self::Error::from(UnexpectedInput))
        }
    }
}
