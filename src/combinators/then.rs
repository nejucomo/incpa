#[cfg(test)]
mod tests;

mod parser;

pub use self::parser::ThenParser;

use derive_new::new;

use crate::{Input, Parser};

/// Parse `P` then `Q`
#[derive(Copy, Clone, Debug, new)]
#[new(visibility = "pub(crate)")]
pub struct Then<P, Q> {
    p: P,
    q: Q,
}

impl<P, Q, I> Parser<I> for Then<P, Q>
where
    I: ?Sized + Input + 'static,
    P: Parser<I>,
    Q: Parser<I, Error = P::Error>,
{
    type Output = (P::Output, Q::Output);
    type Error = P::Error;
    type State = ThenParser<P::State, P::Output, Q::State>;

    fn start_parser(self) -> Self::State {
        ThenParser::new(self.p.start_parser(), self.q.start_parser())
    }
}
