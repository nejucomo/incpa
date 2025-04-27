use std::marker::PhantomData;

use derive_new::new;
use incpa::Parser;

use crate::{ParseRecursiveState, Step};

#[derive(Debug, new)]
pub struct ParseRecursive<P, C, O> {
    inner: P,
    #[new(default)]
    ph: PhantomData<(C, O)>,
}

impl<I, P, C, O> Parser<I> for ParseRecursive<P, C, O>
where
    P: Parser<I, Output = Step<O, C>>,
{
    type Output = O;
    type Error = P::Error;
    type State = ParseRecursiveState<P::State, C, O>;

    fn into_parser(self) -> Self::State {
        ParseRecursiveState::new(self.inner.into_parser())
    }
}
