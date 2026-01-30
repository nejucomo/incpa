use incpa_ioe::IncpaIOE;
use incpa_state::map::MapOutcome as _;
use incpa_state::{ChompedResult, Outcome, ParserState};

use crate::{AutoRecursingState, Continuation};

#[derive(Debug)]
pub struct RecursiveState<S, O>
where
    S: AutoRecursingState<O> + Clone,
{
    start: S,
    state: S,
    cs: Vec<S::Continuation>,
}

impl<S, O> RecursiveState<S, O>
where
    S: AutoRecursingState<O> + Clone,
{
    pub(crate) fn new(state: S) -> Self {
        Self {
            start: state.clone(),
            state,
            cs: vec![],
        }
    }
}

impl<S, O> IncpaIOE for RecursiveState<S, O>
where
    S: AutoRecursingState<O> + Clone,
{
    type Input = S::Input;
    type Output = S::Output;
    type Error = S::Error;
}

impl<S, O> ParserState for RecursiveState<S, O>
where
    S: AutoRecursingState<O> + Clone,
{
    fn feed(self, input: &Self::Input) -> ChompedResult<Outcome<Self, Self::Output>, Self::Error> {
        use crate::RecursingControl::*;
        use Outcome::*;

        let Self {
            start,
            state,
            mut cs,
        } = self;

        state.feed_recursingly(input).map_outcome(|oc| match oc {
            Next(State(state)) => Next(Self { start, state, cs }),
            Next(Cont(c)) => {
                cs.push(c);
                let state = start.clone();
                Next(Self { start, state, cs })
            }
            Parsed(out) => {
                if let Some(c) = cs.pop() {
                    let state = c.continue_with(out);
                    Next(Self { start, state, cs })
                } else {
                    Parsed(out)
                }
            }
        })
    }
}
