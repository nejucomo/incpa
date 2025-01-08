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

impl<P, F, E, I> Syntax<I> for MapError<P, F, E>
where
    P: Syntax<I>,
    F: FnOnce(P::Error) -> E,
    E: From<BaseParserError>,
    I: ?Sized,
{
    type Output = P::Output;
    type Error = E;
    type State = MapErrorParser<P::State, F, E>;

    fn into_parser(self) -> Self::State {
        let MapError { inner, f, .. } = self;

        MapErrorParser(MapError::new(inner.into_parser(), f))
    }
}

#[derive(Copy, Clone, Debug, new)]
#[new(visibility = "pub(crate)")]
pub struct MapErrorParser<P, F, E>(MapError<P, F, E>);

impl<P, F, E, I> Parser<I> for MapErrorParser<P, F, E>
where
    P: Parser<I>,
    F: FnOnce(P::Error) -> E,
    E: From<BaseParserError>,
    I: ?Sized,
{
    type Output = P::Output;
    type Error = E;

    fn feed(self, input: &I) -> Result<Update<Self, Self::Output>, E> {
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
