#[cfg(test)]
mod tests;

use incpa_compose::Or;
use incpa_state::compose::OrState;

use crate::Parser;

impl<P, Q> Parser for Or<P, Q>
where
    P: Parser,
    Q: Parser<Input = P::Input, Output = P::Output, Error = P::Error>,
{
    type State = OrState<P::State, Q::State>;

    fn start_parser(self) -> Self::State {
        OrState::new(self.p.start_parser(), self.q.start_parser())
    }
}
