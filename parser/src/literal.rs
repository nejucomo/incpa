#[cfg(test)]
mod tests;

mod arrayimpl;
mod charimpl;
mod sliceimpl;
mod strimpl;

use std::marker::PhantomData;

use incpa_ioe::{IncpaIOE, Input, UniversalParserError};
use incpa_state::{Chomped, ChompedResult, Outcome, ParserState};

use crate::Parser;

/// A [Literal] is any value which is a [Parser] for itself
///
/// # Example
///
/// ```
/// use incpa_ioe::UniversalParserError;
/// use incpa_parser::{Parser, Literal};
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
pub trait Literal<I>: Sized + Copy + Parser<Input = I, Output = Self>
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
#[derive(Copy, Clone, Debug)]
pub struct LiteralState<I, L>
where
    I: ?Sized,
{
    literal: L,
    ph: PhantomData<Box<I>>,
}

impl<I, L> LiteralState<I, L>
where
    I: ?Sized,
{
    pub(crate) fn new(literal: L) -> Self {
        LiteralState {
            literal,
            ph: PhantomData,
        }
    }
}

impl<I, L> IncpaIOE for LiteralState<I, L>
where
    I: ?Sized + Input,
    L: Literal<I>,
{
    type Input = I;
    type Output = L;
    type Error = L::Error;
}

impl<I, L> ParserState for LiteralState<I, L>
where
    I: ?Sized + Input,
    L: Literal<I>,
{
    fn feed(self, input: &Self::Input) -> ChompedResult<Outcome<Self, L>, Self::Error> {
        use UniversalParserError::UnexpectedInput;
        use incpa_state::Outcome::{Next, Parsed};

        let n = self.literal.literal_len();
        let prefix = input.prefix_up_to(n);

        if prefix.len() < n {
            Ok(Chomped::new(0, Next(self)))
        } else if self.literal.literal_eq(prefix) {
            Ok(Chomped::new(n, Parsed(self.literal)))
        } else {
            Err(Self::Error::from(UnexpectedInput))
        }
    }
}
