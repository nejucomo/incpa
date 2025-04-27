use either::Either;

use crate::state::{Backtrack, Buffer, FeedChomped, OutcomeExt, ParserState};

#[derive(Copy, Clone, Debug)]
pub struct OrParser<P, Q> {
    obp: Option<Backtrack<P>>,
    q: Q,
}

impl<P, Q> OrParser<P, Q> {
    pub(super) fn new(p: P, q: Q) -> Self {
        OrParser {
            obp: Some(Backtrack::new(p)),
            q,
        }
    }
}

impl<P, Q, I> ParserState<I> for OrParser<P, Q>
where
    I: ?Sized + Buffer + 'static,
    P: ParserState<I>,
    Q: ParserState<I, Error = P::Error>,
{
    type Output = Either<P::Output, Q::Output>;
    type Error = P::Error;

    fn feed(self, input: &I) -> Result<FeedChomped<Self, Self::Output>, Self::Error> {
        use Either::{Left, Right};

        let OrParser { obp, q } = self;

        if let Some(bp) = obp {
            let res = bp.feed(input).map_output(Left);
            if res.is_ok() {
                return res.map_parser(|bp| OrParser { obp: Some(bp), q });
            }
            // Else we hit an error, so drop `bp` and fall back to `q`:
        }

        q.feed(input)
            .map_parser(|q| OrParser { obp: None, q })
            .map_output(Right)
    }

    fn end_input(self, final_input: &I) -> Result<Self::Output, Self::Error> {
        use Either::{Left, Right};

        let OrParser { obp, q } = self;

        if let Some(bp) = obp {
            let res = bp.end_input(final_input).map(Left);
            if res.is_ok() {
                return res;
            }
            // Else we hit an error, so drop `bp` and fall back to `q`:
        }

        q.end_input(final_input).map(Right)
    }
}
