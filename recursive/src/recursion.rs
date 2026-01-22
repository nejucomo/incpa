use std::marker::PhantomData;

use incpa::state::Chomped;
use incpa::state::Outcome::{Next, Parsed};
use incpa::{Input, ParserCompose, ParserOutErr, UniversalParserError};

use crate::{ChompedRecOutcome, ContinueState, FeedState, RecOutcome, RecState, RecursingParser};

#[derive(Debug)]
pub struct Recursion<O>(PhantomData<O>);

impl<O> Copy for Recursion<O> {}

impl<O> Clone for Recursion<O> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<O> Default for Recursion<O> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<O, I> RecursingParser<I, O> for Recursion<O>
where
    I: ?Sized + Input,
{
    type FState = Self;

    fn start_recursing_parser(self) -> Self::FState {
        self
    }
}

impl<O> ParserCompose for Recursion<O> {}

impl<O> ParserOutErr for Recursion<O> {
    type Output = O;
    type Error = UniversalParserError;
}

impl<I, O> FeedState<I, O> for Recursion<O>
where
    I: ?Sized + Input,
{
    type CState = Self;

    fn feed_recursing(
        self,
        _: &I,
    ) -> Result<ChompedRecOutcome<Self, Self::CState, Self::Output>, Self::Error> {
        Ok(Chomped::new(0, Next(RecState::new(self, Some(self)))))
    }
}

impl<I, O> ContinueState<I, O> for Recursion<O>
where
    I: ?Sized + Input,
{
    type FState = Self;

    fn continue_with(
        self,
        cval: O,
    ) -> Result<RecOutcome<Self::FState, Self, Self::Output>, Self::Error> {
        Ok(Parsed(cval))
    }
}
