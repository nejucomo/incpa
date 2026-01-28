use either::Either;
use incpa_ioe::{IncpaIOE, Input};
use incpa_state::map::{MapConsumed as _, MapOutcome as _};
use incpa_state::{Chomped, ChompedResult, Outcome, ParserState};

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

impl<P, Q> IncpaIOE for ThenParser<P, P::Output, Q>
where
    P: IncpaIOE,
    Q: IncpaIOE<Input = P::Input, Error = P::Error>,
    P::Input: 'static,
{
    type Input = P::Input;
    type Output = (P::Output, Q::Output);
    type Error = P::Error;
}

impl<P, Q> ParserState for ThenParser<P, P::Output, Q>
where
    P: ParserState,
    Q: ParserState<Input = P::Input, Error = P::Error>,
    P::Input: 'static,
{
    fn feed(self, input: &Self::Input) -> ChompedResult<Outcome<Self, Self::Output>, Self::Error> {
        use Either::{Left, Right};
        use incpa_state::Outcome::{Next, Parsed};

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
            Right(pval) => q.feed(input).map_outcome(|oc| match oc {
                Next(q) => Next(ThenParser {
                    porval: Right(pval),
                    q,
                }),
                Parsed(qval) => Parsed((pval, qval)),
            }),
        }
    }

    fn end_input(self, final_input: &Self::Input) -> Result<Self::Output, Self::Error> {
        let (pval, input) = self.porval.either(
            |p| p.end_input(final_input).map(|pval| (pval, Self::Input::empty())),
            |pval| Ok((pval, final_input)),
        )?;
        let qval = self.q.end_input(input)?;
        Ok((pval, qval))
    }
}
