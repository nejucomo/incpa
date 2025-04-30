use std::marker::PhantomData;

use incpa::Parser;
use incpa::state::Outcome;
use incpa::state::{Chomped, FeedChomped, Outcome::Parsed, ParserState};

use crate::{Continuation, RecursiveParser, Step};

#[derive(Copy, Debug)]
pub struct Recursion<R>(PhantomData<R>);

impl<R> Default for Recursion<R> {
    fn default() -> Self {
        Recursion(PhantomData)
    }
}

impl<R> Clone for Recursion<R> {
    fn clone(&self) -> Self {
        Recursion(PhantomData)
    }
}

impl<I, R> RecursiveParser<I, R, R> for Recursion<R>
where
    I: ?Sized,
{
    type Continuation = Recursion<R>;
}

impl<I, R> Parser<I> for Recursion<R>
where
    I: ?Sized,
{
    type Output = Step<R, Self>;
    type Error = incpa::BaseParserError;
    type State = Self;

    fn into_parser(self) -> Self::State {
        self
    }
}

impl<I, R> ParserState<I> for Recursion<R>
where
    I: ?Sized,
{
    type Output = Step<R, Self>;
    type Error = incpa::BaseParserError;

    fn feed(self, _: &I) -> Result<FeedChomped<Self, Self::Output>, Self::Error> {
        Ok(Chomped::new(0, Parsed(Step::RequestRec(self))))
    }
}

impl<R> Continuation<Recursion<R>, R, R, incpa::BaseParserError> for Recursion<R> {
    fn recurse_from(
        self,
        parsed: R,
    ) -> Result<Outcome<Recursion<R>, Step<R, Self>>, incpa::BaseParserError> {
        Ok(Parsed(Step::ParsedRec(parsed)))
    }
}
