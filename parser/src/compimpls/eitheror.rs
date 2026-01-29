use incpa_compose::EitherOr;
use incpa_state::compose::EitherOrState;

use crate::Parser;

impl<P, Q> Parser for EitherOr<P, Q>
where
    P: Parser,
    Q: Parser<Input = P::Input, Error = P::Error>,
{
    type State = EitherOrState<P::State, Q::State>;

    fn start_parser(self) -> Self::State {
        EitherOrState::new(self.p.start_parser(), self.q.start_parser())
    }
}
