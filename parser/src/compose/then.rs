#[cfg(test)]
mod tests;

mod state;

use derive_new::new;
use incpa_state::Input;

use crate::Parser;

use self::state::ThenState;

/// Parse `P` then `Q`
#[derive(Copy, Clone, Debug, new)]
#[new(visibility = "pub(crate)")]
pub struct Then<P, Q> {
    /// The initial parser
    pub p: P,
    /// The subsequent parser
    pub q: Q,
}

impl<I, P, Q> Parser<I> for Then<P, Q>
where
    I: ?Sized + Input,
    P: Parser<I>,
    Q: Parser<I, Error = P::Error>,
{
    type State = ThenState<P::State, Q::State>;
    type Output = (P::Output, Q::Output);
    type Error = P::Error;

    fn start_parser(self) -> Self::State {
        ThenState::new(self.p.start_parser(), self.q.start_parser())
    }
}
