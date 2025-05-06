use std::marker::PhantomData;

use derive_new::new;

use crate::state::{FeedChomped, OutcomeExt, ParserState};
use crate::{Parser, UniversalParserError};

/// Specifies a parser which maps its error
#[derive(Copy, Clone, Debug, new)]
#[new(visibility = "pub(crate)")]
pub struct MapError<P, F, E> {
    inner: P,
    f: F,
    #[new(default)]
    ph: PhantomData<E>,
}

/// A [ParserState] to map the error
#[derive(Copy, Clone, Debug, new)]
#[new(visibility = "pub(crate)")]
pub struct MapErrorState<S, O, EI, F, EO> {
    inner: S,
    f: F,
    #[new(default)]
    ph: PhantomData<(O, EI, EO)>,
}

impl<I, P, F, E> Parser<I> for MapError<P, F, E>
where
    I: ?Sized,
    P: Parser<I>,
    F: FnOnce(P::Error) -> E,
    E: From<UniversalParserError>,
{
    type Output = P::Output;
    type Error = E;
    type State = MapErrorState<P::State, P::Output, P::Error, F, E>;

    fn start_parser(self) -> Self::State {
        let MapError { inner, f, .. } = self;

        MapErrorState::new(inner.start_parser(), f)
    }
}

impl<I, O, EI, S, F, EO> ParserState<I, O, EO> for MapErrorState<S, O, EI, F, EO>
where
    I: ?Sized,
    S: ParserState<I, O, EI>,
    F: FnOnce(EI) -> EO,
    EI: From<UniversalParserError>,
{
    fn feed(self, input: &I) -> Result<FeedChomped<Self, O>, EO> {
        let MapErrorState { inner, f, .. } = self;

        match inner.feed(input) {
            Ok(up) => Ok(up.map_parser(|p| MapErrorState::new(p, f))),
            Err(e) => Err(f(e)),
        }
    }

    fn end_input(self, final_input: &I) -> Result<O, EO> {
        let MapErrorState { inner, f, .. } = self;

        inner.end_input(final_input).map_err(f)
    }
}
