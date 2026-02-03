use incpa_state::map::MapNext as _;
use incpa_state::{Backtrack, ChompedResult, Outcome, ParserState};

/// The [ParserState] for parsing `P` then `Q`
#[derive(Copy, Clone, Debug)]
pub struct OrState<P, Q> {
    obp: Option<Backtrack<P>>,
    q: Q,
}

impl<P, Q> OrState<P, Q> {
    /// Construct a new [OrState]
    pub fn new(p: P, q: Q) -> Self {
        OrState {
            obp: Some(Backtrack::new(p)),
            q,
        }
    }
}

impl<P, Q> ParserState for OrState<P, Q>
where
    P: ParserState,
    Q: ParserState<Input = P::Input, Output = P::Output, Error = P::Error>,
{
    type Input = P::Input;
    type Output = P::Output;
    type Error = P::Error;

    fn feed(self, input: &Self::Input) -> ChompedResult<Outcome<Self, Self::Output>, Self::Error> {
        let OrState { obp, q } = self;

        if let Some(bp) = obp {
            let res = bp.feed(input);
            if res.is_ok() {
                return res.map_next(|bp| OrState { obp: Some(bp), q });
            }
            // Else we hit an error, so drop `bp` and fall back to `q`:
        }

        q.feed(input).map_next(|q| OrState { obp: None, q })
    }

    fn end_input(self, final_input: &Self::Input) -> Result<Self::Output, Self::Error> {
        let OrState { obp, q } = self;

        if let Some(bp) = obp {
            let res = bp.end_input(final_input);
            if res.is_ok() {
                return res;
            }
            // Else we hit an error, so drop `bp` and fall back to `q`:
        }

        q.end_input(final_input)
    }
}
