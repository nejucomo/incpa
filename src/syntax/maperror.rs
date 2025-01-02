use std::marker::PhantomData;

use derive_new::new;

use crate::parsing::{OutcomeExt, Parser, Update};
use crate::{BaseParserError, Syntax};

/// Specifies a parser which maps its error
#[derive(Copy, Clone, Debug, new)]
#[new(visibility = "pub(crate)")]
pub struct MapError<P, F, E> {
    inner: P,
    f: F,
    #[new(default)]
    ph: PhantomData<E>,
}

impl<P, F, I, O, E, E2> Syntax<I, O, E2> for MapError<P, F, E>
where
    P: Syntax<I, O, E>,
    F: FnOnce(E) -> E2,
    E: From<BaseParserError>,
    E2: From<BaseParserError>,
{
    type State = MapErrorParser<P::State, F, E>;

    fn into_parser(self) -> Self::State {
        let MapError { inner, f, .. } = self;

        MapErrorParser(MapError::new(inner.into_parser(), f))
    }
}

#[derive(Copy, Clone, Debug, new)]
#[new(visibility = "pub(crate)")]
pub struct MapErrorParser<P, F, E>(MapError<P, F, E>);

impl<P, F, I, O, E, E2> Parser<I, O, E2> for MapErrorParser<P, F, E>
where
    P: Parser<I, O, E>,
    F: FnOnce(E) -> E2,
    E: From<BaseParserError>,
    E2: From<BaseParserError>,
{
    fn feed(self, input: &I) -> Result<Update<Self, O>, E2> {
        let MapError { inner, f, .. } = self.0;

        match inner.feed(input) {
            Ok(up) => Ok(up.map_parser(|p| MapErrorParser(MapError::new(p, f)))),
            Err(e) => Err(f(e)),
        }
    }

    fn end_input(self, final_input: &I) -> Result<O, E2> {
        let MapError { inner, f, .. } = self.0;

        inner.end_input(final_input).map_err(f)
    }
}
