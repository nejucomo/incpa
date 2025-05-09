use std::marker::PhantomData;

use crate::state::{FeedChomped, ParserState};
use crate::{Parser, UniversalParserError};

/// An opaque type which parses the inner instances of a recursive grammar
pub struct Recursion<O, E>(PhantomData<(O, E)>);

impl<O, E> Default for Recursion<O, E> {
    fn default() -> Self {
        Recursion(PhantomData)
    }
}

impl<I, O, E> Parser<I> for Recursion<O, E>
where
    I: ?Sized,
    E: From<UniversalParserError>,
{
    type Output = O;
    type Error = E;
    type State = Self;

    fn start_parser(self) -> Self::State {
        self
    }
}

impl<I, O, E> ParserState<I> for Recursion<O, E>
where
    I: ?Sized,
    E: From<UniversalParserError>,
{
    type Output = O;
    type Error = E;

    fn feed(self, input: &I) -> Result<FeedChomped<Self, Self::Output>, Self::Error> {
        let _ = input;
        todo!()
    }
}
