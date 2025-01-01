use either::Either;

use crate::{BaseParserError, Buffer, Parser, Update, UpdateExt};

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

impl<P, Q, I, PO, QO, E> Parser<I, (PO, QO), E> for ThenState<P, PO, Q>
where
    I: ?Sized + Buffer + 'static,
    P: Parser<I, PO, E>,
    Q: Parser<I, QO, E>,
    E: From<BaseParserError>,
{
    fn feed(self, input: &I) -> Result<crate::Update<Self, (PO, QO)>, E> {
        use crate::Outcome::{Next, Parsed};
        use Either::{Left, Right};

        let ThenState { porval, q } = self;

        match porval {
            Left(p) => {
                let Update { consumed, outcome } = p.feed(input)?;

                match outcome {
                    Next(p) => Ok(Update {
                        consumed,
                        outcome: Next(ThenState::new(p, q)),
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

    fn end_input(self, final_input: &I) -> Result<(PO, QO), E> {
        let (pval, input) = self.porval.either(
            |p| p.end_input(final_input).map(|pval| (pval, I::empty())),
            |pval| Ok((pval, final_input)),
        )?;
        let qval = self.q.end_input(input)?;
        Ok((pval, qval))
    }
}
