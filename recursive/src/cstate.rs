use incpa::{Input, ParserOutErr};

use crate::FeedState;
use crate::recstate::RecOutcome;

pub trait ContinueState<I: ?Sized + Input, C>: ParserOutErr {
    type FState: FeedState<I, C, CState = Self, Output = Self::Output, Error = Self::Error>;

    fn continue_with(
        self,
        cval: C,
    ) -> Result<RecOutcome<Self::FState, Self, Self::Output>, Self::Error>;
}
