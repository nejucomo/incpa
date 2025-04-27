use derive_new::new;

use crate::state::{Outcome, OutcomeExt};

/// The [Ok] result of [Parser::feed](crate::Parser::feed)
pub type FeedUpdate<P, O> = Update<Outcome<P, O>>;

/// Tracks a number of elements consumed for a `value`
///
/// The element units are implicitly defined by the `I` input parameter to [Parser](crate::Parser)
#[derive(Debug, PartialEq, new)]
pub struct Update<T> {
    /// The number of input units consumed
    pub consumed: usize,
    /// The associated value
    pub value: T,
}

impl<T, E> Update<Result<T, E>> {
    /// Construct a new [Update]
    pub fn transpose(self) -> Result<Update<T>, E> {
        let Update { consumed, value } = self;
        let value = value?;
        Ok(Update { consumed, value })
    }
}

/// Extension methods to map updates within other structures
pub trait UpdateExt<T> {
    /// The container type produced by mapping the update
    type MappedUpdate<U>;

    /// Map the consumed amount
    fn map_consumed<F>(self, f: F) -> Self::MappedUpdate<T>
    where
        F: FnOnce(usize) -> usize;

    /// Map the outcome
    fn map_outcome<F, U>(self, f: F) -> Self::MappedUpdate<U>
    where
        F: FnOnce(T) -> U;
}

impl<T> UpdateExt<T> for Update<T> {
    type MappedUpdate<U> = Update<U>;

    fn map_consumed<F>(self, f: F) -> Self::MappedUpdate<T>
    where
        F: FnOnce(usize) -> usize,
    {
        Update {
            consumed: f(self.consumed),
            value: self.value,
        }
    }

    fn map_outcome<F, U>(self, f: F) -> Self::MappedUpdate<U>
    where
        F: FnOnce(T) -> U,
    {
        Update {
            consumed: self.consumed,
            value: f(self.value),
        }
    }
}

impl<P, O> OutcomeExt<P, O> for FeedUpdate<P, O> {
    type MappedOutcome<MOP, MOO> = FeedUpdate<MOP, MOO>;

    fn map_parser<F, P2>(self, f: F) -> FeedUpdate<P2, O>
    where
        F: FnOnce(P) -> P2,
    {
        self.map_outcome(|oc| oc.map_parser(f))
    }

    /// Map the output
    fn map_output<F, O2>(self, f: F) -> FeedUpdate<P, O2>
    where
        F: FnOnce(O) -> O2,
    {
        self.map_outcome(|oc| oc.map_output(f))
    }
}
