use std::marker::PhantomData;

use derive_new::new;

use crate::recursive::RecursiveInnerState;
use crate::state::{FeedChomped, ParserState};

/// A [ParserState] which handles internal recursive parsing
#[derive(Debug, new)]
pub struct RecursiveParserState<I, S>
where
    I: ?Sized,
    S: RecursiveInnerState<I>,
{
    inner: S,
    #[new(default)]
    continuations: Vec<S::Continuation>,
    #[new(default)]
    ph: PhantomData<I>,
}

impl<I, S> ParserState<I> for RecursiveParserState<I, S>
where
    I: ?Sized,
    S: RecursiveInnerState<I>,
{
    type Output = S::Output;
    type Error = S::Error;

    fn feed(self, input: &I) -> Result<FeedChomped<Self, Self::Output>, Self::Error> {
        let RecursiveParserState {
            inner,
            continuations,
            ph,
        } = self;
        let _ = (input, inner, continuations, ph);
        todo!();
    }
}
