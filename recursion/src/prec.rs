use std::marker::PhantomData;

use derive_new::new;
use incpa::Parser;

use crate::{RecursionPivot, RecursiveParser};

/// Given a [RecursiveParser] that may request internal recursive parses of the output `O` in order to parse an outer `O`, transform it into a [Parser]
pub fn parse_recursive<P, I, O>(parser: P) -> impl Parser<I, Output = O>
where
    P: RecursiveParser<I, O, O>,
    I: ?Sized,
{
    ParseRecursive::new(parser)
}

/// A wrapper which enables a [RecursiveParser] `P` to provide a [Parser] interface
#[derive(Debug, new)]
pub struct ParseRecursive<P, I, O>
where
    P: RecursiveParser<I, O, O>,
    I: ?Sized,
{
    inner: P,
    #[new(default)]
    ph: PhantomData<(O, I)>,
}

impl<P, I, O> Parser<I> for ParseRecursive<P, I, O>
where
    P: RecursiveParser<I, O, O>,
    I: ?Sized,
{
    type Output = O;
    type Error = P::Error;
    type State = RecursionPivot<P, I, O>;

    fn into_parser(self) -> Self::State {
        RecursionPivot::from(self.inner)
    }
}
