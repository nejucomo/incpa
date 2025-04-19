use std::marker::PhantomData;

use derive_new::new;

use crate::Parser;
use crate::parsing::{ParserState, Update, UpdateExt};

/// Specifies a parser which maps its output
#[derive(Copy, Clone, Debug, new)]
#[new(visibility = "pub(crate)")]
pub struct MapOutput<P, F, O> {
    inner: P,
    f: F,
    #[new(default)]
    ph: PhantomData<O>,
}

impl<P, F, O, I> Parser<I> for MapOutput<P, F, O>
where
    P: Parser<I>,
    F: FnOnce(P::Output) -> O,
{
    type Output = O;
    type Error = P::Error;
    type State = MapOutputParser<P::State, F, O>;

    fn into_parser(self) -> Self::State {
        MapOutputParser(MapOutput::new(self.inner.into_parser(), self.f))
    }
}

/// A parser which maps its output
#[derive(Copy, Clone, Debug, new)]
#[new(visibility = "pub(crate)")]
pub struct MapOutputParser<P, F, O>(MapOutput<P, F, O>);

impl<P, F, I, O> ParserState<I> for MapOutputParser<P, F, O>
where
    P: ParserState<I>,
    F: FnOnce(P::Output) -> O,
{
    type Output = O;
    type Error = P::Error;

    fn feed(self, input: &I) -> Result<Update<Self, O>, Self::Error> {
        use crate::parsing::Outcome::{Next, Parsed};

        let MapOutput { inner, f, .. } = self.0;

        inner.feed(input).map_outcome(|oc| match oc {
            Next(p) => Next(MapOutputParser(MapOutput::new(p, f))),
            Parsed(o) => Parsed(f(o)),
        })
    }

    fn end_input(self, final_input: &I) -> Result<O, Self::Error> {
        self.0.inner.end_input(final_input).map(self.0.f)
    }
}
