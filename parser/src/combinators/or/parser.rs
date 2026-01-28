use either::Either;
use incpa_ioe::IncpaIOE;
use incpa_state::map::{MapNext as _, MapParsed as _};
use incpa_state::{Backtrack, ChompedResult, Outcome, ParserState};

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

impl<P, Q> IncpaIOE for OrParser<P, Q>
where
    P: IncpaIOE,
    Q: IncpaIOE<Input = P::Input, Error = P::Error>,
{
    type Input = P::Input;
    type Output = Either<P::Output, Q::Output>;
    type Error = P::Error;
}

impl<P, Q> ParserState for OrParser<P, Q>
where
    P: ParserState,
    Q: ParserState<Input = P::Input, Error = P::Error>,
{
    fn feed(self, input: &Self::Input) -> ChompedResult<Outcome<Self, Self::Output>, Self::Error> {
        use Either::{Left, Right};

        let OrParser { obp, q } = self;

        if let Some(bp) = obp {
            let res = bp.feed(input).map_parsed(Left);
            if res.is_ok() {
                return res.map_next(|bp| OrParser { obp: Some(bp), q });
            }
            // Else we hit an error, so drop `bp` and fall back to `q`:
        }

        q.feed(input)
            .map_next(|q| OrParser { obp: None, q })
            .map_parsed(Right)
    }

    fn end_input(self, final_input: &Self::Input) -> Result<Self::Output, Self::Error> {
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
