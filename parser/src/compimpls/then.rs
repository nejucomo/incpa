#[cfg(test)]
mod tests;

use incpa_compose::Then;
use incpa_state::compose::ThenState;

use crate::Parser;

impl<P, Q> Parser for Then<P, Q>
where
    P: Parser,
    Q: Parser<Input = P::Input, Error = P::Error>,
    P::Input: 'static,
{
    type State = ThenState<P::State, Q::State>;

    fn start_parser(self) -> Self::State {
        ThenState::new(self.p.start_parser(), self.q.start_parser())
    }
}
