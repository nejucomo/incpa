#[cfg(test)]
mod tests;

mod state;

pub use self::state::ThenState;

use derive_new::new;

use crate::state::Buffer;
use crate::{Parser, ParserCombinator, ParserOutput};

/// Parse `P` then `Q`
#[derive(Copy, Clone, Debug, new)]
#[new(visibility = "pub(crate)")]
pub struct Then<P, Q> {
    p: P,
    q: Q,
}

impl<P, Q> ParserOutput for Then<P, Q>
where
    P: ParserOutput,
    Q: ParserOutput<Error = P::Error>,
{
    type Output = (P::Output, Q::Output);
    type Error = P::Error;
}

impl<P, Q> ParserCombinator for Then<P, Q>
where
    P: ParserOutput,
    Q: ParserOutput<Error = P::Error>,
{
}

impl<P, Q, I> Parser<I> for Then<P, Q>
where
    I: ?Sized + Buffer + 'static,
    P: Parser<I>,
    Q: Parser<I, Error = P::Error>,
{
    type State = ThenState<P::State, P::Output, Q::State>;

    fn start_parser(self) -> Self::State {
        ThenState::new(self.p.start_parser(), self.q.start_parser())
    }
}
