use crate::recursive::Continuation;
use crate::state::ParserState;

/// A [ParserState] with an associated [Continuation]
pub trait RecursiveInnerState<I>: ParserState<I>
where
    I: ?Sized,
{
    /// The associated [Continuation] for this state
    type Continuation: Continuation;
}
