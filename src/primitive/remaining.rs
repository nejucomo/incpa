use std::marker::PhantomData;

use crate::{Parser, Update};

/// Captures all remaining input
///
/// # Warning
///
/// This requires holding all input in memory, by definition.
pub fn remaining<I, E>() -> impl Parser<I, I::Owned, E>
where
    I: ToOwned,
{
    Remaining(PhantomData)
}

struct Remaining<I, E>(PhantomData<(I, E)>);

impl<I, E> Parser<I, I::Owned, E> for Remaining<I, E>
where
    I: ToOwned,
{
    fn feed(self, _: &I) -> Result<Update<Self, I::Owned>, E> {
        use crate::Outcome::Next;

        Ok(Update {
            consumed: 0,
            outcome: Next(self),
        })
    }

    fn unwrap_pending(self, final_input: &I) -> Option<I::Owned> {
        Some(final_input.to_owned())
    }
}
