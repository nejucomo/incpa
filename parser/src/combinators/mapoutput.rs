use std::marker::PhantomData;

use derive_new::new;
use incpa_state::map::MapOutcome as _;
use incpa_state::{ChompedResult, Input, Outcome, ParserState};

use crate::{Parser, ParserCompose};

/// Specifies a parser which maps its output
#[derive(Copy, Clone, Debug, new)]
#[new(visibility = "pub(crate)")]
pub struct MapOutput<P, F, O> {
    inner: P,
    f: F,
    #[new(default)]
    ph: PhantomData<O>,
}

impl<P, F, O> ParserCompose for MapOutput<P, F, O>
where
    P: ParserCompose,
    F: FnOnce(P::Output) -> O,
{
    type Output = O;
    type Error = P::Error;
}

impl<P, F, O, I> Parser<I> for MapOutput<P, F, O>
where
    I: ?Sized + Input,
    P: Parser<I>,
    F: FnOnce(P::Output) -> O,
{
    type State = MapOutputParser<P::State, F, O>;

    fn start_parser(self) -> Self::State {
        MapOutputParser(MapOutput::new(self.inner.start_parser(), self.f))
    }
}

/// A parser which maps its output
#[derive(Copy, Clone, Debug, new)]
#[new(visibility = "pub(crate)")]
pub struct MapOutputParser<P, F, O>(MapOutput<P, F, O>);

impl<P, F, I, O> ParserState<I> for MapOutputParser<P, F, O>
where
    I: ?Sized + Input,
    P: ParserState<I>,
    F: FnOnce(P::Output) -> O,
{
    type Output = O;
    type Error = P::Error;

    fn feed(self, input: &I) -> ChompedResult<Outcome<Self, O>, Self::Error> {
        use incpa_state::Outcome::{Next, Parsed};

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
