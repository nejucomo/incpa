use std::marker::PhantomData;

use derive_new::new;

use crate::{ParseResultExt, Parser, Update};

/// A parser which maps its output
#[derive(Copy, Clone, Debug, new)]
#[new(visibility = "pub(crate)")]
pub struct MapOutput<P, F, O> {
    inner: P,
    f: F,
    #[new(default)]
    ph: PhantomData<O>,
}

impl<P, F, I, O, O2, E> Parser<I, O2, E> for MapOutput<P, F, O>
where
    P: Parser<I, O, E>,
    F: FnOnce(O) -> O2,
{
    fn feed(self, input: &I) -> Result<Update<Self, O2>, E> {
        use crate::Outcome::{Next, Parsed};

        let MapOutput { inner, f, .. } = self;

        inner.feed(input).map_outcome(|oc| match oc {
            Next(p) => Next(p.map(f)),
            Parsed(o) => Parsed(f(o)),
        })
    }

    fn unwrap_pending(self) -> Option<O2> {
        self.inner.unwrap_pending().map(self.f)
    }
}
