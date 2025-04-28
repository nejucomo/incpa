use std::marker::PhantomData;

use derive_new::new;
use incpa::Parser;

use crate::{Continuation, RecursionPivot, Step};

/// Given a [RecursiveParser] that may request internal recursive parses of the output `O` in order to parse an outer `O`, transform it into a [Parser]
pub fn parse_recursive<P, I, O>(parser: P) -> impl Parser<I, Output = O>
where
    P: RecursiveParser<I, O, O>,
    I: ?Sized,
{
    ParseRecursive::new(parser)
}

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

/// A [RecursiveParser] is a [Parser] which either produces an output, or a request for recursively parsing `R` and a [Continuation] to continue with the resulting `R` value
pub trait RecursiveParser<I, R, O>:
    Clone + Parser<I, Output = Step<O, Self::Continuation>>
where
    I: ?Sized,
{
    /// Given a recursively parsed `R` value, take the next parsing step for our `O` output
    type Continuation: Continuation<Self::State, R, O, Self::Error>;
}
