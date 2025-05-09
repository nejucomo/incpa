use std::marker::PhantomData;

use derive_new::new;

use crate::Parser;
use crate::state::{ChompedExt, FeedChomped, ParserState};

/// Specifies a parser which maps its output
#[derive(Copy, Clone, Debug, new)]
#[new(visibility = "pub(crate)")]
pub struct MapOutput<I, P, F, O>
where
    I: ?Sized,
{
    inner: P,
    f: F,
    #[new(default)]
    ph: PhantomData<(O, I)>,
}

impl<I, P, F, O> Parser<I> for MapOutput<I, P, F, O>
where
    I: ?Sized,
    P: Parser<I>,
    F: FnOnce(P::Output) -> O,
{
    type Output = O;
    type Error = P::Error;
    type State = MapOutputParser<I, P::State, F, O>;

    fn start_parser(self) -> Self::State {
        MapOutputParser(MapOutput::new(self.inner.start_parser(), self.f))
    }
}

/// A parser which maps its output
#[derive(Copy, Clone, Debug, new)]
#[new(visibility = "pub(crate)")]
pub struct MapOutputParser<I, S, F, O>(MapOutput<I, S, F, O>)
where
    I: ?Sized;

impl<I, S, F, O> ParserState<I> for MapOutputParser<I, S, F, O>
where
    I: ?Sized,
    S: ParserState<I>,
    F: FnOnce(S::Output) -> O,
{
    type Output = O;
    type Error = S::Error;

    fn feed(self, input: &I) -> Result<FeedChomped<Self, O>, Self::Error> {
        use crate::state::Outcome::{Next, Parsed};

        let MapOutput { inner, f, .. } = self.0;

        inner.feed(input).map_value(|oc| match oc {
            Next(p) => Next(MapOutputParser(MapOutput::new(p, f))),
            Parsed(o) => Parsed(f(o)),
        })
    }

    fn end_input(self, final_input: &I) -> Result<O, Self::Error> {
        self.0.inner.end_input(final_input).map(self.0.f)
    }
}
