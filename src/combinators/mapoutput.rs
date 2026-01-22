use std::marker::PhantomData;

use derive_new::new;

use crate::state::{ChompedExt, FeedChomped, ParserState};
use crate::{Input, Parser, ParserCompose, ParserOutErr};

/// Specifies a parser which maps its output
#[derive(Copy, Clone, Debug, new)]
#[new(visibility = "pub")]
pub struct MapOutput<P, F, O> {
    /// The inner parser
    pub inner: P,
    /// The mapping function
    pub f: F,
    #[new(default)]
    ph: PhantomData<O>,
}

impl<P, F, O> ParserOutErr for MapOutput<P, F, O>
where
    P: ParserCompose,
    F: FnOnce(P::Output) -> O,
{
    type Output = O;
    type Error = P::Error;
}

impl<P, F, O> ParserCompose for MapOutput<P, F, O>
where
    P: ParserCompose,
    F: FnOnce(P::Output) -> O,
{
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

impl<P, F, O> ParserOutErr for MapOutputParser<P, F, O>
where
    P: ParserOutErr,
    F: FnOnce(P::Output) -> O,
{
    type Output = O;
    type Error = P::Error;
}

impl<P, F, I, O> ParserState<I> for MapOutputParser<P, F, O>
where
    I: ?Sized + Input,
    P: ParserState<I>,
    F: FnOnce(P::Output) -> O,
{
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
