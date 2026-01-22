use incpa::{Input, ParserCompose};

use crate::FeedState;

pub trait RecursingParser<I: ?Sized + Input, O>: ParserCompose {
    type FState: FeedState<I, O, Output = Self::Output, Error = Self::Error>;

    fn start_recursing_parser(self) -> Self::FState;
}
