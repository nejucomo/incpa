use std::marker::PhantomData;

use derive_new::new;

use crate::parsing::{Parser, Update, UpdateExt};
use crate::{BaseParserError, Syntax};

/// A parser which maps its output
#[derive(Copy, Clone, Debug, new)]
#[new(visibility = "pub(crate)")]
pub struct MapOutput<P, F, O> {
    inner: P,
    f: F,
    #[new(default)]
    ph: PhantomData<O>,
}

impl<P, F, I, O, O2, E> Syntax<I, O2, E> for MapOutput<P, F, O>
where
    P: Syntax<I, O, E>,
    F: FnOnce(O) -> O2,
    E: From<BaseParserError>,
{
    type State = MapOutput<P::State, F, O>;

    fn into_parser(self) -> Self::State {
        MapOutput::new(self.inner.into_parser(), self.f)
    }
}

impl<P, F, I, O, O2, E> Parser<I, O2, E> for MapOutput<P, F, O>
where
    P: Parser<I, O, E>,
    F: FnOnce(O) -> O2,
    E: From<BaseParserError>,
{
    fn feed(self, input: &I) -> Result<Update<Self, O2>, E> {
        use crate::parsing::Outcome::{Next, Parsed};

        let MapOutput { inner, f, .. } = self;

        inner.feed(input).map_outcome(|oc| match oc {
            Next(p) => Next(MapOutput::new(p, f)),
            Parsed(o) => Parsed(f(o)),
        })
    }

    fn end_input(self, final_input: &I) -> Result<O2, E> {
        self.inner.end_input(final_input).map(self.f)
    }
}
