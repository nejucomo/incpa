use crate::state::{ChompedExt, OutcomeExt};

impl<K, T, E> ChompedExt<T> for Result<K, E>
where
    K: ChompedExt<T>,
{
    type MapChomp<U> = Result<K::MapChomp<U>, E>;

    fn map_consumed<F>(self, f: F) -> Self::MapChomp<T>
    where
        F: FnOnce(usize) -> usize,
    {
        self.map(|k| k.map_consumed(f))
    }

    fn map_value<F, U>(self, f: F) -> Self::MapChomp<U>
    where
        F: FnOnce(T) -> U,
    {
        self.map(|k| k.map_value(f))
    }
}

impl<K, P, O, E> OutcomeExt<P, O> for Result<K, E>
where
    K: OutcomeExt<P, O>,
{
    type MappedOutcome<P2, O2> = Result<<K as OutcomeExt<P, O>>::MappedOutcome<P2, O2>, E>;

    fn map_parser<F, P2>(self, f: F) -> Self::MappedOutcome<P2, O>
    where
        F: FnOnce(P) -> P2,
    {
        self.map(|up| up.map_parser(f))
    }

    fn map_output<F, O2>(self, f: F) -> Self::MappedOutcome<P, O2>
    where
        F: FnOnce(O) -> O2,
    {
        self.map(|up| up.map_output(f))
    }
}
