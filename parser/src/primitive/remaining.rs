#[cfg(test)]
mod tests;

use std::fmt::Debug;
use std::marker::PhantomData;

use incpa_compose::ParserCompose;
use incpa_ioe::{IncpaIOE, Input, UniversalParserError};
use incpa_state::{Chomped, ChompedResult, Outcome, ParserState};

use crate::Parser;

/// Captures all remaining input
///
/// # Warning
///
/// This requires holding all input in memory, by definition.
pub fn remaining<I>()
-> impl Parser<Input = I, Output = I::Owned, Error = UniversalParserError> + Copy + Debug
where
    I: ?Sized + Input + ToOwned,
{
    Remaining(PhantomData)
}

struct Remaining<I>(PhantomData<Box<I>>)
where
    I: ?Sized + Input + ToOwned;

impl<I> IncpaIOE for Remaining<I>
where
    I: ?Sized + Input + ToOwned,
{
    type Input = I;
    type Output = I::Owned;
    type Error = UniversalParserError;
}

impl<I> ParserCompose for Remaining<I> where I: ?Sized + Input + ToOwned {}

impl<I> Parser for Remaining<I>
where
    I: ?Sized + Input + ToOwned,
{
    type State = Remaining<I>;

    fn start_parser(self) -> Self::State {
        Remaining(PhantomData)
    }
}

impl<I> ParserState for Remaining<I>
where
    I: ?Sized + Input + ToOwned,
{
    fn feed(self, _: &Self::Input) -> ChompedResult<Outcome<Self, I::Owned>, Self::Error> {
        use incpa_state::Outcome::Next;

        Ok(Chomped::new(0, Next(self)))
    }

    fn end_input(self, final_input: &Self::Input) -> Result<I::Owned, Self::Error> {
        Ok(final_input.to_owned())
    }
}

impl<I> Copy for Remaining<I> where I: ?Sized + Input + ToOwned {}

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
