use either::Either;

use crate::state::{Chomped, ChompedExt, FeedChomped, ParserState};
use crate::{Input, ParserOutErr};

#[derive(Copy, Clone, Debug)]
pub struct ThenParser<P, O, Q> {
    porval: Either<P, O>,
    q: Q,
}

impl<P, O, Q> ThenParser<P, O, Q> {
    pub(super) fn new(p: P, q: Q) -> Self {
        ThenParser {
            porval: Either::Left(p),
            q,
        }
    }
}

impl<P, Q> ParserOutErr for ThenParser<P, P::Output, Q>
where
    P: ParserOutErr,
    Q: ParserOutErr<Error = P::Error>,
{
    type Output = (P::Output, Q::Output);
    type Error = P::Error;
}

impl<P, Q, I> ParserState<I> for ThenParser<P, P::Output, Q>
where
    I: ?Sized + Input,
    P: ParserState<I>,
    Q: ParserState<I, Error = P::Error>,
{
    fn feed(self, input: &I) -> Result<FeedChomped<Self, Self::Output>, Self::Error> {
        use crate::state::Outcome::{Next, Parsed};
        use Either::{Left, Right};

        let ThenParser { porval, q } = self;

        match porval {
            Left(p) => {
                let Chomped {
                    consumed,
                    value: outcome,
                } = p.feed(input)?;

                match outcome {
                    Next(p) => Ok(Chomped {
                        consumed,
                        value: Next(ThenParser::new(p, q)),
                    }),
                    Parsed(pval) => ThenParser {
                        porval: Right(pval),
                        q,
                    }
                    .feed(input.drop_prefix(consumed))
                    .map_consumed(|c| c + consumed),
                }
            }
            Right(pval) => q.feed(input).map_value(|oc| match oc {
                Next(q) => Next(ThenParser {
                    porval: Right(pval),
                    q,
                }),
                Parsed(qval) => Parsed((pval, qval)),
            }),
        }
    }

    fn end_input(self, final_input: &I) -> Result<Self::Output, Self::Error> {
        let empty = final_input.drop_prefix(final_input.len());
        let (pval, input) = self.porval.either(
            |p| p.end_input(final_input).map(|pval| (pval, empty)),
            |pval| Ok((pval, final_input)),
        )?;
        let qval = self.q.end_input(input)?;
        Ok((pval, qval))
    }
}
