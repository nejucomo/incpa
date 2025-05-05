use std::marker::PhantomData;

use derive_new::new;

use crate::state::{ChompedExt, FeedChomped, ParserState};
use crate::{Parser, ParserCombinator, ParserOutput};

/// Specifies a parser which maps its output
#[derive(Copy, Clone, Debug, new)]
#[new(visibility = "pub(crate)")]
pub struct MapOutput<P, F, O> {
    inner: P,
    f: F,
    #[new(default)]
    ph: PhantomData<O>,
}

impl<P, F, O> ParserOutput for MapOutput<P, F, O>
where
    P: ParserOutput,
    F: FnOnce(P::Output) -> O,
{
    type Output = O;
    type Error = P::Error;
}

impl<P, F, O> ParserCombinator for MapOutput<P, F, O>
where
    P: ParserCombinator,
    F: FnOnce(P::Output) -> O,
{
}

impl<P, F, O, I> Parser<I> for MapOutput<P, F, O>
where
    P: Parser<I>,
    F: FnOnce(P::Output) -> O,
{
    type State = MapOutput<P::State, F, O>;

    fn start_parser(self) -> Self::State {
        MapOutput::new(self.inner.start_parser(), self.f)
    }
}

impl<P, F, I, O> ParserState<I> for MapOutput<P, F, O>
where
    P: ParserState<I>,
    F: FnOnce(P::Output) -> O,
{
    fn feed(self, input: &I) -> Result<FeedChomped<Self, O>, Self::Error> {
        use crate::state::Outcome::{Next, Parsed};

        let MapOutput { inner, f, .. } = self;

        inner.feed(input).map_value(|oc| match oc {
            Next(p) => Next(MapOutput::new(p, f)),
            Parsed(o) => Parsed(f(o)),
        })
    }

    fn end_input(self, final_input: &I) -> Result<O, Self::Error> {
        self.inner.end_input(final_input).map(self.f)
    }
}
