use either::Either;

use crate::parsing::{Buffer, Parser, Update, UpdateExt};
use crate::BaseParserError;

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

impl<P, Q, I, PO, QO, E> Parser<I, (PO, QO), E> for ThenParser<P, PO, Q>
where
    I: ?Sized + Buffer + 'static,
    P: Parser<I, PO, E>,
    Q: Parser<I, QO, E>,
    E: From<BaseParserError>,
{
    fn feed(self, input: &I) -> Result<Update<Self, (PO, QO)>, E> {
        use crate::parsing::Outcome::{Next, Parsed};
        use Either::{Left, Right};

        let ThenParser { porval, q } = self;

        match porval {
            Left(p) => {
                let Update { consumed, outcome } = p.feed(input)?;

                match outcome {
                    Next(p) => Ok(Update {
                        consumed,
                        outcome: Next(ThenParser::new(p, q)),
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

    fn end_input(self, final_input: &I) -> Result<(PO, QO), E> {
        let (pval, input) = self.porval.either(
            |p| p.end_input(final_input).map(|pval| (pval, I::empty())),
            |pval| Ok((pval, final_input)),
        )?;
        let qval = self.q.end_input(input)?;
        Ok((pval, qval))
    }
}
