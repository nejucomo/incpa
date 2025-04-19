use crate::state::{Outcome, OutcomeExt, UpdateExt};

impl<K, P, O, E> UpdateExt<P, O> for Result<K, E>
where
    K: UpdateExt<P, O>,
{
    type MappedUpdate<P2, O2> = Result<<K as UpdateExt<P, O>>::MappedUpdate<P2, O2>, E>;

    fn map_consumed<F>(self, f: F) -> Self::MappedUpdate<P, O>
    where
        F: FnOnce(usize) -> usize,
    {
        self.map(|up| up.map_consumed(f))
    }

    fn map_outcome<F, P2, O2>(self, f: F) -> Self::MappedUpdate<P2, O2>
    where
        F: FnOnce(Outcome<P, O>) -> Outcome<P2, O2>,
    {
        self.map(|up| up.map_outcome(f))
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
