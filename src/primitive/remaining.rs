#[cfg(test)]
mod tests;

use std::fmt::Debug;
use std::marker::PhantomData;

use crate::state::{Chomped, FeedChomped, ParserState};
use crate::{Input, Parser, ParserCompose, UniversalParserError};

/// Captures all remaining input
///
/// # Warning
///
/// This requires holding all input in memory, by definition.
pub fn remaining<I>()
-> impl Parser<I, Output = I::Owned, Error = UniversalParserError> + Copy + Debug
where
    I: ?Sized + Input + ToOwned,
{
    Remaining(PhantomData)
}

struct Remaining<I>(PhantomData<Box<I>>)
where
    I: ?Sized + Input + ToOwned;

impl<I> ParserCompose for Remaining<I>
where
    I: ?Sized + Input + ToOwned,
{
    type Output = I::Owned;
    type Error = UniversalParserError;
}

impl<I> Parser<I> for Remaining<I>
where
    I: ?Sized + Input + ToOwned,
{
    type State = Remaining<I>;

    fn start_parser(self) -> Self::State {
        Remaining(PhantomData)
    }
}

impl<I> ParserState<I> for Remaining<I>
where
    I: ?Sized + Input + ToOwned,
{
    type Output = I::Owned;
    type Error = UniversalParserError;

    fn feed(self, _: &I) -> Result<FeedChomped<Self, I::Owned>, Self::Error> {
        use crate::state::Outcome::Next;

        Ok(Chomped::new(0, Next(self)))
    }

    fn end_input(self, final_input: &I) -> Result<I::Owned, Self::Error> {
        Ok(final_input.to_owned())
    }
}

impl<I> Copy for Remaining<I>
where
    I: ?Sized + Input + ToOwned,
{}

impl<I> Clone for Remaining<I>
where
    I: ?Sized + Input + ToOwned,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<I> Debug for Remaining<I>
where
    I: ?Sized + Input + ToOwned,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "remaining<I = {}>()", std::any::type_name::<I>())
    }
}
