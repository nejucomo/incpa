use incpa_compose::MapError;
use incpa_ioe::UniversalParserError;

use crate::Parser;

impl<P, F, E> Parser for MapError<P, F, E>
where
    P: Parser,
    F: FnOnce(P::Error) -> E,
    E: From<UniversalParserError>,
{
    type State = MapError<P::State, F, E>;

    fn start_parser(self) -> Self::State {
        let MapError { inner, f, .. } = self;

        MapError::new(inner.start_parser(), f)
    }
}
