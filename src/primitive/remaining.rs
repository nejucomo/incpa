#[cfg(test)]
mod tests;

use std::fmt::Debug;
use std::marker::PhantomData;

use crate::state::{Chomped, FeedChomped, ParserState};
use crate::{Input, Parser, UniversalParserError};

/// Captures all remaining input
///
/// # Warning
///
/// This requires holding all input in memory, by definition.
pub fn remaining<I: ?Sized + Input + ToOwned + 'static>()
-> impl Parser<I, Output = I::Owned, Error = UniversalParserError> + Copy + Debug {
    Remaining(PhantomData)
}

struct Remaining<I: ?Sized + Input + ToOwned + 'static>(PhantomData<&'static I>);

impl<I: ?Sized + Input + ToOwned + 'static> Parser<I> for Remaining<I> {
    type Output = I::Owned;
    type Error = UniversalParserError;
    type State = Remaining<I>;

    fn start_parser(self) -> Self::State {
        Remaining(PhantomData)
    }
}

impl<I: ?Sized + Input + ToOwned> ParserState<I> for Remaining<I> {
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

impl<I: ?Sized + Input + ToOwned> Copy for Remaining<I> {}

impl<I: ?Sized + Input + ToOwned> Clone for Remaining<I> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<I: ?Sized + Input + ToOwned> Debug for Remaining<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "remaining<I = {}>()", std::any::type_name::<I>())
    }
}
