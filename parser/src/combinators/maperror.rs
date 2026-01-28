use std::marker::PhantomData;

use derive_new::new;
use incpa_ioe::{IncpaIOE, UniversalParserError};
use incpa_state::map::MapNext as _;
use incpa_state::{ChompedResult, Outcome, ParserState};

use crate::{Parser, ParserCompose};

/// Specifies a parser which maps its error
#[derive(Copy, Clone, Debug, new)]
#[new(visibility = "pub(crate)")]
pub struct MapError<P, F, E> {
    inner: P,
    f: F,
    #[new(default)]
    ph: PhantomData<E>,
}

impl<P, F, E> IncpaIOE for MapError<P, F, E>
where
    P: IncpaIOE,
    F: FnOnce(P::Error) -> E,
    E: From<UniversalParserError>,
{
    type Input = P::Input;
    type Output = P::Output;
    type Error = E;
}

impl<P, F, E> ParserCompose for MapError<P, F, E>
where
    P: ParserCompose,
    F: FnOnce(P::Error) -> E,
    E: From<UniversalParserError>,
{
}

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
