use incpa::UniversalParserError;
use incpa::state::FeedChomped;

use crate::RecursionStatus;

/// The [Ok] result of [RecursiveParserState::feed_recursive]
pub type FeedChompedRecursionStatus<S, C, O> = FeedChomped<RecursionStatus<S, C>, O>;

/// Analogous to [incpa::state::ParserState] extended to enable recursive parsing of `R` values
pub trait RecursiveParserState<I, R>: Sized
where
    I: ?Sized,
{
    /// The type of output on successful parse
    type Output;

    /// The type of errors this parser detects
    type Error: From<UniversalParserError>;

    /// The type of continuation which handles recursively parsed values
    type Continuation;

    /// Feed an input reference to this recursive parser to produce an update
    ///
    /// Precondition: `input` includes a suffix which has not been seen previously by this parser.
    fn feed_recursive(
        self,
        input: &I,
    ) -> Result<FeedChompedRecursionStatus<Self, Self::Continuation, Self::Output>, Self::Error>;
}
