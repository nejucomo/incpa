#[cfg(test)]
mod tests;

mod parser;

pub use self::parser::ThenParser;

use derive_new::new;

use crate::parsing::Buffer;
use crate::Syntax;

/// Parse `P` then `Q`
#[derive(Copy, Clone, Debug, new)]
#[new(visibility = "pub(crate)")]
pub struct Then<P, Q> {
    p: P,
    q: Q,
}

impl<P, Q, I> Syntax<I> for Then<P, Q>
where
    I: ?Sized + Buffer + 'static,
    P: Syntax<I>,
    Q: Syntax<I, Error = P::Error>,
{
    type Output = (P::Output, Q::Output);
    type Error = P::Error;
    type State = ThenParser<P::State, P::Output, Q::State>;

    fn into_parser(self) -> Self::State {
        ThenParser::new(self.p.into_parser(), self.q.into_parser())
    }
}
