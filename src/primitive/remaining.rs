#[cfg(test)]
mod tests;

use std::fmt::Debug;
use std::marker::PhantomData;

use crate::state::{Chomped, FeedChomped, ParserState};
use crate::{BaseParserError, Parser};

/// Captures all remaining input
///
/// # Warning
///
/// This requires holding all input in memory, by definition.
pub fn remaining<I>() -> impl Parser<I, Output = I::Owned, Error = BaseParserError> + Copy + Debug
where
    I: ?Sized + ToOwned + 'static,
{
    Remaining(PhantomData)
}

struct Remaining<I>(PhantomData<&'static I>)
where
    I: ?Sized + 'static;

impl<I> Parser<I> for Remaining<I>
where
    I: ?Sized + ToOwned + 'static,
{
    type Output = I::Owned;
    type Error = BaseParserError;
    type State = Remaining<I>;

    fn start_parser(self) -> Self::State {
        Remaining(PhantomData)
    }
}

impl<I> ParserState<I> for Remaining<I>
where
    I: ?Sized + ToOwned + 'static,
{
    type Output = I::Owned;
    type Error = BaseParserError;

    fn feed(self, _: &I) -> Result<FeedChomped<Self, I::Owned>, Self::Error> {
        use crate::state::Outcome::Next;

        Ok(Chomped::new(0, Next(self)))
    }

    fn end_input(self, final_input: &I) -> Result<I::Owned, Self::Error> {
        Ok(final_input.to_owned())
    }
}

impl<I> Copy for Remaining<I> where I: ?Sized {}

impl<I> Clone for Remaining<I>
where
    I: ?Sized,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<I> Debug for Remaining<I>
where
    I: ?Sized + ToOwned + 'static,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "remaining<I = {}>()", std::any::type_name::<I>())
    }
}
