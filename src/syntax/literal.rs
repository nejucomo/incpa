#[cfg(test)]
mod tests;

mod arrayimpl;
mod charimpl;
mod sliceimpl;
mod strimpl;

use derive_new::new;

use crate::parsing::{Buffer, ParserState, Update};
use crate::Parser;

/// A [Literal] is any value which is syntax for a parser of itself
///
/// # Example
///
/// ```
/// use incpa::BaseParserError;
/// use incpa::syntax::{Parser,Literal};
///
/// fn main() -> Result<(), BaseParserError> {
///   // &str is a Literal, so it can parse an input:
///   let syntax = "Hello World!";
///   let parsed = syntax.parse_all("Hello World!")?;
///   // A literal syntax parses itself to output itself:
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

    fn feed(self, input: &I) -> Result<Update<Self, L>, Self::Error> {
        use crate::parsing::Outcome::{Next, Parsed};
        use crate::BaseParserError::UnexpectedInput;

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
