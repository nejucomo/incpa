use derive_new::new;

use crate::state::{Outcome, OutcomeExt};

/// The [Ok] result of [ParserState::feed](crate::state::ParserState::feed)
pub type FeedChomped<P, O> = Chomped<Outcome<P, O>>;

/// Tracks a number of elements consumed for a `value`
///
/// The element units are implicitly defined by the `I` input parameter to [Parser](crate::Parser)
#[derive(Debug, PartialEq, new)]
pub struct Chomped<T> {
    /// The number of input units consumed
    pub consumed: usize,
    /// The associated value
    pub value: T,
}

impl<T, E> Chomped<Result<T, E>> {
    /// Transpose a [Chomped] [Result]
    pub fn transpose(self) -> Result<Chomped<T>, E> {
        let Chomped { consumed, value } = self;
        let value = value?;
        Ok(Chomped { consumed, value })
    }
}

/// Extension methods to map updates within other structures
pub trait ChompedExt<T> {
    /// The container type produced by mapping the update
    type MapChomp<U>;

    /// Map the consumed amount
    fn map_consumed<F>(self, f: F) -> Self::MapChomp<T>
    where
        F: FnOnce(usize) -> usize;

    /// Map the outcome
    fn map_value<F, U>(self, f: F) -> Self::MapChomp<U>
    where
        F: FnOnce(T) -> U;
}

impl<T> ChompedExt<T> for Chomped<T> {
    type MapChomp<U> = Chomped<U>;

    fn map_consumed<F>(self, f: F) -> Self::MapChomp<T>
    where
        F: FnOnce(usize) -> usize,
    {
        Chomped {
            consumed: f(self.consumed),
            value: self.value,
        }
    }

    fn map_value<F, U>(self, f: F) -> Self::MapChomp<U>
    where
        F: FnOnce(T) -> U,
    {
        Chomped {
            consumed: self.consumed,
            value: f(self.value),
        }
    }
}

impl<P, O> OutcomeExt<P, O> for FeedChomped<P, O> {
    type MappedOutcome<MOP, MOO> = FeedChomped<MOP, MOO>;

    fn map_parser<F, P2>(self, f: F) -> FeedChomped<P2, O>
    where
        F: FnOnce(P) -> P2,
    {
        self.map_value(|oc| oc.map_parser(f))
    }

    /// Map the output
    fn map_output<F, O2>(self, f: F) -> FeedChomped<P, O2>
    where
        F: FnOnce(O) -> O2,
    {
        self.map_value(|oc| oc.map_output(f))
    }
}
