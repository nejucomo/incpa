use either::Either;

use crate::ParserOutput;
use crate::state::{Buffer, Chomped, ChompedExt, FeedChomped, ParserState};

#[derive(Copy, Clone, Debug)]
pub struct ThenState<P, O, Q> {
    porval: Either<P, O>,
    q: Q,
}

impl<P, O, Q> ThenState<P, O, Q> {
    pub(super) fn new(p: P, q: Q) -> Self {
        ThenState {
            porval: Either::Left(p),
            q,
        }
    }
}

impl<P, Q> ParserOutput for ThenState<P, P::Output, Q>
where
    P: ParserOutput,
    Q: ParserOutput<Error = P::Error>,
{
    type Output = (P::Output, Q::Output);
    type Error = P::Error;
}

impl<P, Q, I> ParserState<I> for ThenState<P, P::Output, Q>
where
    I: ?Sized + Buffer + 'static,
    P: ParserState<I>,
    Q: ParserState<I, Error = P::Error>,
{
    fn feed(self, input: &I) -> Result<FeedChomped<Self, Self::Output>, Self::Error> {
        use crate::state::Outcome::{Next, Parsed};
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
            Right(pval) => q.feed(input).map_value(|oc| match oc {
                Next(q) => Next(ThenState {
                    porval: Right(pval),
                    q,
                }),
                Parsed(qval) => Parsed((pval, qval)),
            }),
        }
    }

    fn end_input(self, final_input: &I) -> Result<Self::Output, Self::Error> {
        let (pval, input) = self.porval.either(
            |p| p.end_input(final_input).map(|pval| (pval, I::empty())),
            |pval| Ok((pval, final_input)),
        )?;
        let qval = self.q.end_input(input)?;
        Ok((pval, qval))
    }
}
