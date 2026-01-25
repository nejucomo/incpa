use crate::map::{MapConsumed, MapNext, MapOutcome, MapParsed};

impl<T, E, M> MapNext<M> for Result<T, E>
where
    T: MapNext<M>,
{
    type MappedNext<N> = Result<<T as MapNext<M>>::MappedNext<N>, E>;

    fn map_next<F, N>(self, f: F) -> Self::MappedNext<N>
    where
        F: FnOnce(M) -> N,
    {
        self.map(|t| t.map_next(f))
    }
}

impl<T, E, M> MapParsed<M> for Result<T, E>
where
    T: MapParsed<M>,
{
    type MappedParsed<N> = Result<<T as MapParsed<M>>::MappedParsed<N>, E>;

    fn map_parsed<F, N>(self, f: F) -> Self::MappedParsed<N>
    where
        F: FnOnce(M) -> N,
    {
        self.map(|t| t.map_parsed(f))
    }
}

impl<T, E> MapConsumed for Result<T, E>
where
    T: MapConsumed,
{
    fn map_consumed<F>(self, f: F) -> Self
    where
        F: FnOnce(usize) -> usize,
    {
        self.map(|t| t.map_consumed(f))
    }
}

impl<T, E, M> MapOutcome<M> for Result<T, E>
where
    T: MapOutcome<M>,
{
    type MappedOutcome<N> = Result<<T as MapOutcome<M>>::MappedOutcome<N>, E>;

    fn map_outcome<F, N>(self, f: F) -> Self::MappedOutcome<N>
    where
        F: FnOnce(M) -> N,
    {
        self.map(|t| t.map_outcome(f))
    }
}
