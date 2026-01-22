use incpa::{Input, ParserOutErr};

use crate::{ChompedRecOutcome, ContinueState};

pub trait FeedState<I: ?Sized + Input, C>: ParserOutErr {
    type CState: ContinueState<I, C, FState = Self, Output = Self::Output, Error = Self::Error>;

    /// Feed an input reference to the parser to produce an update
    ///
    /// Precondition: `input` includes a suffix which has not been seen previously by this parser.
    fn feed_recursing(
        self,
        input: &I,
    ) -> Result<ChompedRecOutcome<Self, Self::CState, Self::Output>, Self::Error>;
}

pub trait AutoFeedState<I: ?Sized + Input>: FeedState<I, <Self as ParserOutErr>::Output> {}

impl<B, I: ?Sized + Input> AutoFeedState<I> for B where
    B: FeedState<I, <Self as ParserOutErr>::Output>
{
}
