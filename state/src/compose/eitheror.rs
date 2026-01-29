use either::Either::{self, Left, Right};
use incpa_ioe::IncpaIOE;

use crate::map::{MapNext as _, MapParsed as _};
use crate::{Backtrack, ChompedResult, Outcome, ParserState};

/// The [ParserState] for parsing `P` then `Q`
#[derive(Copy, Clone, Debug)]
pub struct EitherOrState<P, Q> {
    obp: Option<Backtrack<P>>,
    q: Q,
}

impl<P, Q> EitherOrState<P, Q> {
    /// Construct a new [EitherOrState]
    pub fn new(p: P, q: Q) -> Self {
        EitherOrState {
            obp: Some(Backtrack::new(p)),
            q,
        }
    }
}

impl<P, Q> IncpaIOE for EitherOrState<P, Q>
where
    P: IncpaIOE,
    Q: IncpaIOE<Input = P::Input, Error = P::Error>,
{
    type Input = P::Input;
    type Output = Either<P::Output, Q::Output>;
    type Error = P::Error;
}

impl<P, Q> ParserState for EitherOrState<P, Q>
where
    P: ParserState,
    Q: ParserState<Input = P::Input, Error = P::Error>,
{
    fn feed(self, input: &Self::Input) -> ChompedResult<Outcome<Self, Self::Output>, Self::Error> {
        let EitherOrState { obp, q } = self;

        if let Some(bp) = obp {
            let res = bp.feed(input);
            if res.is_ok() {
                return res
                    .map_next(|bp| EitherOrState { obp: Some(bp), q })
                    .map_parsed(Left);
            }
            // Else we hit an error, so drop `bp` and fall back to `q`:
        }

        q.feed(input)
            .map_next(|q| EitherOrState { obp: None, q })
            .map_parsed(Right)
    }

    fn end_input(self, final_input: &Self::Input) -> Result<Self::Output, Self::Error> {
        let EitherOrState { obp, q } = self;

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
