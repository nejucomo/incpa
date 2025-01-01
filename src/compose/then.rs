#[cfg(test)]
mod tests;

use either::Either;

use crate::{Buffer, Parser, Update, UpdateExt};

/// Parses `P` then `Q`
#[derive(Copy, Clone, Debug)]
pub struct Then<P, O, Q> {
    porval: Either<P, O>,
    q: Q,
}

impl<P, O, Q> Then<P, O, Q> {
    pub(crate) fn new(p: P, q: Q) -> Self {
        Then {
            porval: Either::Left(p),
            q,
        }
    }
}

impl<P, Q, I, PO, QO, E> Parser<I, (PO, QO), E> for Then<P, PO, Q>
where
    I: ?Sized + Buffer + 'static,
    P: Parser<I, PO, E>,
    Q: Parser<I, QO, E>,
{
    fn feed(self, input: &I) -> Result<crate::Update<Self, (PO, QO)>, E> {
        use crate::Outcome::{Next, Parsed};
        use Either::{Left, Right};

        let Then { porval, q } = self;

        match porval {
            Left(p) => {
                let Update { consumed, outcome } = p.feed(input)?;

                match outcome {
                    Next(p) => Ok(Update {
                        consumed,
                        outcome: Next(Then::new(p, q)),
                    }),
                    Parsed(pval) => Then {
                        porval: Right(pval),
                        q,
                    }
                    .feed(input.drop_prefix(consumed))
                    .map_consumed(|c| c + consumed),
                }
            }
            Right(pval) => q.feed(input).map_outcome(|oc| match oc {
                Next(q) => Next(Then {
                    porval: Right(pval),
                    q,
                }),
                Parsed(qval) => Parsed((pval, qval)),
            }),
        }
    }

    fn unwrap_pending(self, final_input: &I) -> Option<(PO, QO)> {
        let (pval, input) = self.porval.either(
            |p| p.unwrap_pending(final_input).map(|pval| (pval, I::empty())),
            |pval| Some((pval, final_input)),
        )?;
        let qval = self.q.unwrap_pending(input)?;
        Some((pval, qval))
    }
}
