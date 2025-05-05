#[cfg(test)]
mod tests;

mod state;

pub use self::state::OrState;

use derive_new::new;
use either::Either;

use crate::state::Buffer;
use crate::{Parser, ParserCombinator, ParserOutput};

/// Parse `P` or if that fails, parse `Q`
///
/// This holds all input while parsing `P`.
#[derive(Copy, Clone, Debug, new)]
#[new(visibility = "pub(crate)")]
pub struct Or<P, Q> {
    p: P,
    q: Q,
}

impl<P, Q> ParserOutput for Or<P, Q>
where
    P: ParserOutput,
    Q: ParserOutput<Error = P::Error>,
{
    type Output = Either<P::Output, Q::Output>;
    type Error = P::Error;
}

impl<P, Q> ParserCombinator for Or<P, Q>
where
    P: ParserCombinator,
    Q: ParserCombinator<Error = P::Error>,
{
}

impl<P, Q, I> Parser<I> for Or<P, Q>
where
    I: ?Sized + Buffer + 'static,
    P: Parser<I>,
    Q: Parser<I, Error = P::Error>,
{
    type State = OrState<P::State, Q::State>;

    fn start_parser(self) -> Self::State {
        OrState::new(self.p.start_parser(), self.q.start_parser())
    }
}
