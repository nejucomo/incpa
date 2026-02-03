#[cfg(test)]
mod tests;

mod arrayimpl;
mod charimpl;
mod sliceimpl;
mod state;
mod strimpl;

use incpa_state::Input;

use crate::Parser;

/// A [Literal] is any value which is a [Parser] for itself
///
/// # Example
///
/// ```
/// use incpa_state::UniversalParserError;
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
