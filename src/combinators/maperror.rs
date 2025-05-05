use std::marker::PhantomData;

use derive_new::new;

use crate::state::{FeedChomped, OutcomeExt, ParserState};
use crate::{Parser, ParserCombinator, ParserOutput, UniversalParserError};

/// Specifies a parser which maps its error
#[derive(Copy, Clone, Debug, new)]
#[new(visibility = "pub(crate)")]
pub struct MapError<P, F, E> {
    inner: P,
    f: F,
    #[new(default)]
    ph: PhantomData<E>,
}

impl<P, F, E> ParserOutput for MapError<P, F, E>
where
    P: ParserOutput,
    F: FnOnce(P::Error) -> E,
    E: From<UniversalParserError>,
{
    type Output = P::Output;
    type Error = E;
}

impl<P, F, E> ParserCombinator for MapError<P, F, E>
where
    P: ParserCombinator,
    F: FnOnce(P::Error) -> E,
    E: From<UniversalParserError>,
{
}

impl<P, F, E, I> Parser<I> for MapError<P, F, E>
where
    P: Parser<I>,
    F: FnOnce(P::Error) -> E,
    E: From<UniversalParserError>,
    I: ?Sized,
{
    type State = MapError<P::State, F, E>;

    fn start_parser(self) -> Self::State {
        let MapError { inner, f, .. } = self;

        MapError::new(inner.start_parser(), f)
    }
}

impl<P, F, E, I> ParserState<I> for MapError<P, F, E>
where
    P: ParserState<I>,
    F: FnOnce(P::Error) -> E,
    E: From<UniversalParserError>,
    I: ?Sized,
{
    fn feed(self, input: &I) -> Result<FeedChomped<Self, Self::Output>, E> {
        let MapError { inner, f, .. } = self;

        match inner.feed(input) {
            Ok(up) => Ok(up.map_parser(|p| MapError::new(p, f))),
            Err(e) => Err(f(e)),
        }
    }

    fn end_input(self, final_input: &I) -> Result<Self::Output, E> {
        let MapError { inner, f, .. } = self;

        inner.end_input(final_input).map_err(f)
    }
}
