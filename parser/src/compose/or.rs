#[cfg(test)]
mod tests;

mod state;

use derive_new::new;
use incpa_state::Input;

use crate::Parser;
use crate::compose::or::state::OrState;

/// Parse `P` or if that fails, parse `Q`, where both produce the same output type
///
/// This holds all input while parsing `P`.
#[derive(Copy, Clone, Debug, new)]
pub struct Or<P, Q> {
    /// The primary parser, which is attempted first
    pub p: P,
    /// The alternative parser
    pub q: Q,
}

impl<I, P, Q> Parser<I> for Or<P, Q>
where
    I: ?Sized + Input,
    P: Parser<I>,
    Q: Parser<I, Output = P::Output, Error = P::Error>,
{
    type State = OrState<P::State, Q::State>;
    type Output = P::Output;
    type Error = P::Error;

    fn start_parser(self) -> Self::State {
        OrState::new(self.p.start_parser(), self.q.start_parser())
    }
}
