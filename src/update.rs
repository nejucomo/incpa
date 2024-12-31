use crate::Outcome;

/// Provides the number of elements consumed and the outcome
#[derive(Debug, PartialEq)]
pub struct Update<S, O> {
    /// The number of input units consumed
    pub consumed: usize,
    /// The outcome of incremental parsing
    pub outcome: Outcome<S, O>,
}

impl<S, O> Update<S, O> {
    /// Construct a new [Update]
    pub fn new(consumed: usize, outcome: Outcome<S, O>) -> Self {
        Update { consumed, outcome }
    }

    /// Record consuming a single item
    pub fn parsed(consumed: usize, parsed: O) -> Self {
        Self::new(consumed, Outcome::Parsed(parsed))
    }

    /// Map the outcome
    pub fn map_outcome<F, S2, O2>(self, f: F) -> Update<S2, O2>
    where
        F: FnOnce(Outcome<S, O>) -> Outcome<S2, O2>,
    {
        Update {
            consumed: self.consumed,
            outcome: f(self.outcome),
        }
    }

    /// Map the pending state
    pub fn map_next<F, S2>(self, f: F) -> Update<S2, O>
    where
        F: FnOnce(S) -> S2,
    {
        self.map_outcome(|oc| oc.map_next(f))
    }

    /// Map the output
    pub fn map_output<F, O2>(self, f: F) -> Update<S, O2>
    where
        F: FnOnce(O) -> O2,
    {
        self.map_outcome(|oc| oc.map_output(f))
    }
}
