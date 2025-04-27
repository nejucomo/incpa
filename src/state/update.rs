use crate::state::{Outcome, OutcomeExt};

/// Provides the number of elements consumed and the outcome
#[derive(Debug, PartialEq)]
pub struct Update<P, O> {
    /// The number of input units consumed
    pub consumed: usize,
    /// The outcome of incremental parsing
    pub outcome: Outcome<P, O>,
}

impl<P, O> Update<P, O> {
    /// Construct a new [Update]
    pub fn new(consumed: usize, outcome: Outcome<P, O>) -> Self {
        Update { consumed, outcome }
    }
}

/// Extension methods to map updates within other structures
pub trait UpdateExt<P, O, E> {
    /// The container type produced by mapping the update
    type MappedUpdate<P2, O2>;

    /// The container type produced by trying to map the update
    type TryMappedUpdate<P2, O2, E2>;

    /// Map the consumed amount
    fn map_consumed<F>(self, f: F) -> Self::MappedUpdate<P, O>
    where
        F: FnOnce(usize) -> usize;

    /// Map the outcome
    fn map_outcome<F, P2, O2>(self, f: F) -> Self::MappedUpdate<P2, O2>
    where
        F: FnOnce(Outcome<P, O>) -> Outcome<P2, O2>;

    /// Map the outcome, propagating errors
    fn try_map_outcome<F, P2, O2>(self, f: F) -> Self::TryMappedUpdate<P2, O2, E>
    where
        F: FnOnce(Outcome<P, O>) -> Result<Outcome<P2, O2>, E>;
}

impl<P, O, E> UpdateExt<P, O, E> for Update<P, O> {
    type MappedUpdate<P2, O2> = Update<P2, O2>;
    type TryMappedUpdate<P2, O2, E2> = Result<Update<P2, O2>, E2>;

    fn map_consumed<F>(self, f: F) -> Self::MappedUpdate<P, O>
    where
        F: FnOnce(usize) -> usize,
    {
        Update::new(f(self.consumed), self.outcome)
    }

    fn map_outcome<F, P2, O2>(self, f: F) -> Update<P2, O2>
    where
        F: FnOnce(Outcome<P, O>) -> Outcome<P2, O2>,
    {
        Update {
            consumed: self.consumed,
            outcome: f(self.outcome),
        }
    }

    fn try_map_outcome<F, P2, O2>(self, f: F) -> Result<Self::MappedUpdate<P2, O2>, E>
    where
        F: FnOnce(Outcome<P, O>) -> Result<Outcome<P2, O2>, E>,
    {
        let Update { consumed, outcome } = self;
        let outcome = f(outcome)?;
        Ok(Update { consumed, outcome })
    }
}

impl<P, O> OutcomeExt<P, O> for Update<P, O> {
    type MappedOutcome<MOP, MOO> = Update<MOP, MOO>;

    fn map_parser<F, P2>(self, f: F) -> Update<P2, O>
    where
        F: FnOnce(P) -> P2,
    {
        self.map_outcome(|oc| oc.map_parser(f))
    }

    /// Map the output
    fn map_output<F, O2>(self, f: F) -> Update<P, O2>
    where
        F: FnOnce(O) -> O2,
    {
        self.map_outcome(|oc| oc.map_output(f))
    }
}
