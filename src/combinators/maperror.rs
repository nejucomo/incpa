use std::marker::PhantomData;

use derive_new::new;

use crate::state::{FeedChomped, OutcomeExt, ParserState};
use crate::{Parser, UniversalParserError};

/// Specifies a parser which maps its error
#[derive(Copy, Clone, Debug, new)]
#[new(visibility = "pub(crate)")]
pub struct MapError<I, P, F, E>
where
    I: ?Sized,
{
    inner: P,
    f: F,
    #[new(default)]
    ph: PhantomData<(E, I)>,
}

impl<I, P, F, E> Parser<I> for MapError<I, P, F, E>
where
    P: Parser<I>,
    F: FnOnce(P::Error) -> E,
    E: From<UniversalParserError>,
    I: ?Sized,
{
    type Output = P::Output;
    type Error = E;
    type State = MapErrorParser<I, P::State, F, E>;

    fn start_parser(self) -> Self::State {
        let MapError { inner, f, .. } = self;

        MapErrorParser(MapError::new(inner.start_parser(), f))
    }
}

#[derive(Copy, Clone, Debug, new)]
#[new(visibility = "pub(crate)")]
pub struct MapErrorParser<I, S, F, E>(MapError<I, S, F, E>)
where
    I: ?Sized;

impl<I, P, F, E> ParserState<I> for MapErrorParser<I, P, F, E>
where
    I: ?Sized,
    P: ParserState<I>,
    F: FnOnce(P::Error) -> E,
    E: From<UniversalParserError>,
{
    type Output = P::Output;
    type Error = E;

    fn feed(self, input: &I) -> Result<FeedChomped<Self, Self::Output>, E> {
        let MapError { inner, f, .. } = self.0;

        match inner.feed(input) {
            Ok(up) => Ok(up.map_parser(|p| MapErrorParser(MapError::new(p, f)))),
            Err(e) => Err(f(e)),
        }
    }

    fn end_input(self, final_input: &I) -> Result<Self::Output, E> {
        let MapError { inner, f, .. } = self.0;

        inner.end_input(final_input).map_err(f)
    }
}
