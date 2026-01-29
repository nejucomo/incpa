use incpa_ioe::IncpaIOE;

use crate::compose::EitherOrState;
use crate::map::{MapNext as _, MapParsed as _};
use crate::{ChompedResult, Outcome, ParserState};

/// The [ParserState] for parsing `P` then `Q`
#[derive(Copy, Clone, Debug)]
pub struct OrState<P, Q> {
    eost: EitherOrState<P, Q>,
}

impl<P, Q> OrState<P, Q> {
    /// Construct a new [OrState]
    pub fn new(p: P, q: Q) -> Self {
        OrState {
            eost: EitherOrState::new(p, q),
        }
    }
}

impl<P, Q> IncpaIOE for OrState<P, Q>
where
    P: IncpaIOE,
    Q: IncpaIOE<Input = P::Input, Output = P::Output, Error = P::Error>,
{
    type Input = P::Input;
    type Output = P::Output;
    type Error = P::Error;
}

impl<P, Q> ParserState for OrState<P, Q>
where
    P: ParserState,
    Q: ParserState<Input = P::Input, Output = P::Output, Error = P::Error>,
{
    fn feed(self, input: &Self::Input) -> ChompedResult<Outcome<Self, Self::Output>, Self::Error> {
        self.eost
            .feed(input)
            .map_next(|eost| OrState { eost })
            .map_parsed(|ei| ei.into_inner())
    }

    fn end_input(self, final_input: &Self::Input) -> Result<Self::Output, Self::Error> {
        self.eost.end_input(final_input).map(|ei| ei.into_inner())
    }
}
