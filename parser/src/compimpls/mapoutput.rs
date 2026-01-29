use incpa_compose::MapOutput;

use crate::Parser;

impl<P, F, O> Parser for MapOutput<P, F, O>
where
    P: Parser,
    F: FnOnce(P::Output) -> O,
{
    type State = MapOutput<P::State, F, O>;

    fn start_parser(self) -> Self::State {
        MapOutput::new(self.inner.start_parser(), self.f)
    }
}
