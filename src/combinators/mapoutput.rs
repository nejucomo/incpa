use std::marker::PhantomData;

use derive_new::new;

use crate::state::{ChompedExt, FeedChomped, ParserState};
use crate::{Parser, UniversalParserError};

/// Specifies a parser which maps its output
#[derive(Copy, Clone, Debug, new)]
#[new(visibility = "pub(crate)")]
pub struct MapOutput<P, F, O> {
    inner: P,
    f: F,
    #[new(default)]
    ph: PhantomData<O>,
}

/// A parser which maps its output
#[derive(Copy, Clone, Debug, new)]
#[new(visibility = "pub(crate)")]
pub struct MapOutputState<S, OI, E, F, OO> {
    inner: S,
    f: F,
    #[new(default)]
    ph: PhantomData<(OI, OO, E)>,
}

impl<P, F, O, I> Parser<I> for MapOutput<P, F, O>
where
    P: Parser<I>,
    F: FnOnce(P::Output) -> O,
{
    type Output = O;
    type Error = P::Error;
    type State = MapOutputState<P::State, P::Output, P::Error, F, O>;

    fn start_parser(self) -> Self::State {
        MapOutputState::new(self.inner.start_parser(), self.f)
    }
}

impl<I, S, OI, E, F, OO> ParserState<I, OO, E> for MapOutputState<S, OI, E, F, OO>
where
    I: ?Sized,
    S: ParserState<I, OI, E>,
    F: FnOnce(OI) -> OO,
    E: From<UniversalParserError>,
{
    fn feed(self, input: &I) -> Result<FeedChomped<Self, OO>, E> {
        use crate::state::Outcome::{Next, Parsed};

        let MapOutputState { inner, f, .. } = self;

        inner.feed(input).map_value(|oc| match oc {
            Next(p) => Next(MapOutputState::new(p, f)),
            Parsed(o) => Parsed(f(o)),
        })
    }

    fn end_input(self, final_input: &I) -> Result<OO, E> {
        self.inner.end_input(final_input).map(self.f)
    }
}
