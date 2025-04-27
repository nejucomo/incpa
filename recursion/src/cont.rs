use incpa::state::Outcome;

use crate::Step;

/// The complement to [`RecursiveParser::State`](crate::RecursiveParser::State) which produces the next [Outcome] given a recursively parsed `R`
pub trait Continuation<S, R, O, E>: Sized {
    fn recurse_from(self, parsed: R) -> Result<Outcome<S, Step<O, Self>>, E>;
}
