use incpa::Parser;

use crate::{Continuation, Step};

/// A [RecursiveParser] is a [Parser] which either produces an output, or a request for recursively parsing `R` and a [Continuation] to continue with the resulting `R` value
pub trait RecursiveParser<I, R, O>:
    Clone + Parser<I, Output = Step<O, Self::Continuation>>
where
    I: ?Sized,
{
    /// Given a recursively parsed `R` value, take the next parsing step for our `O` output
    type Continuation: Continuation<Self::State, R, O, Self::Error>;
}
