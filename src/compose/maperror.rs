use std::marker::PhantomData;

use derive_new::new;

use crate::{OutcomeExt, Parser, Update};

/// A parser which maps its output
#[derive(Copy, Clone, Debug, new)]
#[new(visibility = "pub(crate)")]
pub struct MapError<P, F, E> {
    inner: P,
    f: F,
    #[new(default)]
    ph: PhantomData<E>,
}

impl<P, F, I, O, E, E2> Parser<I, O, E2> for MapError<P, F, E>
where
    P: Parser<I, O, E>,
    F: FnOnce(E) -> E2,
{
    fn feed(self, input: &I) -> Result<Update<Self, O>, E2> {
        let MapError { inner, f, .. } = self;

        match inner.feed(input) {
            Ok(up) => Ok(up.map_parser(|p| MapError::new(p, f))),
            Err(e) => Err(f(e)),
        }
    }

    fn unwrap_pending(self, final_input: &I) -> Option<O> {
        self.inner.unwrap_pending(final_input)
    }
}
