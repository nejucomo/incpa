use derive_new::new;

use crate::map::{MapConsumed, MapNext, MapOutcome, MapParsed};
use crate::state::Outcome;

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

impl<T> MapConsumed for Chomped<T> {
    fn map_consumed<F>(self, f: F) -> Self
    where
        F: FnOnce(usize) -> usize,
    {
        Chomped {
            consumed: f(self.consumed),
            value: self.value,
        }
    }
}

impl<T> MapOutcome<T> for Chomped<T> {
    type MappedOutcome<U> = Chomped<U>;

    fn map_outcome<F, U>(self, f: F) -> Self::MappedOutcome<U>
    where
        F: FnOnce(T) -> U,
    {
        Chomped {
            consumed: self.consumed,
            value: f(self.value),
        }
    }
}

impl<T, N> MapNext<N> for Chomped<T>
where
    T: MapNext<N>,
{
    type MappedNext<O> = Chomped<<T as MapNext<N>>::MappedNext<O>>;

    fn map_next<F, U>(self, f: F) -> Self::MappedNext<U>
    where
        F: FnOnce(N) -> U,
    {
        self.map_outcome(|oc| oc.map_next(f))
    }
}

impl<T, P> MapParsed<P> for Chomped<T>
where
    T: MapParsed<P>,
{
    type MappedParsed<Q> = Chomped<<T as MapParsed<P>>::MappedParsed<Q>>;

    fn map_parsed<F, U>(self, f: F) -> Self::MappedParsed<U>
    where
        F: FnOnce(P) -> U,
    {
        self.map_outcome(|oc| oc.map_parsed(f))
    }
}
