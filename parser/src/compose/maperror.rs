use std::marker::PhantomData;

use derive_new::new;
use incpa_state::map::MapNext as _;
use incpa_state::{ChompedResult, Input, Outcome, ParserState, UniversalParserError};

use crate::Parser;

/// Specifies a parser which maps its error
#[derive(Copy, Clone, Debug, new)]
pub struct MapError<P, F, E> {
    /// The inner parser or state
    pub inner: P,
    /// The error mapping function
    pub f: F,
    #[new(default)]
    ph: PhantomData<E>,
}

impl<I, P, F, E> Parser<I> for MapError<P, F, E>
where
    I: ?Sized + Input,
    P: Parser<I>,
    F: FnOnce(P::Error) -> E,
    E: From<UniversalParserError>,
{
    type State = MapError<P::State, F, E>;
    type Output = P::Output;
    type Error = E;

    fn start_parser(self) -> Self::State {
        todo!()
    }
}

impl<S, F, E> ParserState for MapError<S, F, E>
where
    S: ParserState,
    F: FnOnce(S::Error) -> E,
    E: From<UniversalParserError>,
{
    type Input = S::Input;
    type Output = S::Output;
    type Error = E;

    fn feed(self, input: &Self::Input) -> ChompedResult<Outcome<MapError<S, F, E>, S::Output>, E> {
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
