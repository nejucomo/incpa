use incpa_compose::MapError;
use incpa_ioe::UniversalParserError;

use crate::map::MapNext as _;
use crate::{ChompedResult, Outcome, ParserState};

impl<P, F, E> ParserState for MapError<P, F, E>
where
    P: ParserState,
    F: FnOnce(P::Error) -> E,
    E: From<UniversalParserError>,
{
    fn feed(self, input: &Self::Input) -> ChompedResult<Outcome<Self, Self::Output>, E> {
        let MapError { inner, f, .. } = self;

        match inner.feed(input) {
            Ok(up) => Ok(up.map_next(|p| MapError::new(p, f))),
            Err(e) => Err(f(e)),
        }
    }

    fn end_input(self, final_input: &Self::Input) -> Result<Self::Output, E> {
        let MapError { inner, f, .. } = self;

        inner.end_input(final_input).map_err(f)
    }
}
