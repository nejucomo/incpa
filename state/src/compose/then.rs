use either::Either;
use incpa_ioe::{IncpaIOE, Input};

use crate::map::{MapConsumed as _, MapOutcome as _};
use crate::{Chomped, ChompedResult, Outcome, ParserState};

/// The state of parsing `P` then `Q`
#[derive(Copy, Clone, Debug)]
pub struct ThenState<P, Q>
where
    P: IncpaIOE,
    Q: IncpaIOE<Input = P::Input, Error = P::Error>,
{
    porval: Either<P, P::Output>,
    q: Q,
}

impl<P, Q> ThenState<P, Q>
where
    P: IncpaIOE,
    Q: IncpaIOE<Input = P::Input, Error = P::Error>,
{
    /// Construct a new [ThenState]
    pub fn new(p: P, q: Q) -> Self {
        ThenState {
            porval: Either::Left(p),
            q,
        }
    }
}

impl<P, Q> IncpaIOE for ThenState<P, Q>
where
    P: IncpaIOE,
    Q: IncpaIOE<Input = P::Input, Error = P::Error>,
{
    type Input = P::Input;
    type Output = (P::Output, Q::Output);
    type Error = P::Error;
}

impl<P, Q> ParserState for ThenState<P, Q>
where
    P: ParserState,
    Q: ParserState<Input = P::Input, Error = P::Error>,
{
    fn feed(self, input: &Self::Input) -> ChompedResult<Outcome<Self, Self::Output>, Self::Error> {
        use crate::Outcome::{Next, Parsed};
        use Either::{Left, Right};

        let ThenState { porval, q } = self;

        match porval {
            Left(p) => {
                let Chomped {
                    consumed,
                    value: outcome,
                } = p.feed(input)?;

                match outcome {
                    Next(p) => Ok(Chomped {
                        consumed,
                        value: Next(ThenState::new(p, q)),
                    }),
                    Parsed(pval) => ThenState {
                        porval: Right(pval),
                        q,
                    }
                    .feed(input.drop_prefix(consumed))
                    .map_consumed(|c| c + consumed),
                }
            }
            Right(pval) => q.feed(input).map_outcome(|oc| match oc {
                Next(q) => Next(ThenState {
                    porval: Right(pval),
                    q,
                }),
                Parsed(qval) => Parsed((pval, qval)),
            }),
        }
    }

    fn end_input(self, final_input: &Self::Input) -> Result<Self::Output, Self::Error> {
        let (pval, input) = self.porval.either(
            |p| {
                p.end_input(final_input)
                    .map(|pval| (pval, final_input.empty_suffix()))
            },
            |pval| Ok((pval, final_input)),
        )?;
        let qval = self.q.end_input(input)?;
        Ok((pval, qval))
    }
}
