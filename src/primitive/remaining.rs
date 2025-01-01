#[cfg(test)]
mod tests;

use std::fmt::Debug;
use std::marker::PhantomData;

use crate::{BaseParserError, Parser, Syntax, Update};

/// Captures all remaining input
///
/// # Warning
///
/// This requires holding all input in memory, by definition.
pub fn remaining<I, E>() -> impl Syntax<I, I::Owned, E> + Copy + Debug
where
    I: ?Sized + ToOwned + 'static,
    E: From<BaseParserError>,
{
    Remaining(PhantomData)
}

struct Remaining<I, E>(PhantomData<(&'static I, E)>)
where
    I: ?Sized + 'static;

impl<I, E> Syntax<I, I::Owned, E> for Remaining<I, E>
where
    I: ?Sized + ToOwned + 'static,
    E: From<BaseParserError>,
{
    type State = Remaining<I, E>;

    fn into_parser(self) -> Self::State {
        Remaining(PhantomData)
    }
}

impl<I, E> Parser<I, I::Owned, E> for Remaining<I, E>
where
    I: ?Sized + ToOwned + 'static,
    E: From<BaseParserError>,
{
    fn feed(self, _: &I) -> Result<Update<Self, I::Owned>, E> {
        use crate::Outcome::Next;

        Ok(Update {
            consumed: 0,
            outcome: Next(self),
        })
    }

    fn end_input(self, final_input: &I) -> Result<I::Owned, E> {
        Ok(final_input.to_owned())
    }
}

impl<I, E> Copy for Remaining<I, E> where I: ?Sized {}

impl<I, E> Clone for Remaining<I, E>
where
    I: ?Sized,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<I, E> Debug for Remaining<I, E>
where
    I: ?Sized + ToOwned + 'static,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "remaining<I = {}, E = {}>()",
            std::any::type_name::<I>(),
            std::any::type_name::<E>()
        )
    }
}
