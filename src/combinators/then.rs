#[cfg(test)]
mod tests;

mod parser;

use std::marker::PhantomData;

pub use self::parser::ThenParser;

use derive_new::new;

use crate::Parser;
use crate::state::Buffer;

/// Parse `P` then `Q`
#[derive(Copy, Clone, Debug, new)]
#[new(visibility = "pub(crate)")]
pub struct Then<I, P, Q>
where
    I: ?Sized + Buffer + 'static,
    P: Parser<I>,
    Q: Parser<I, Error = P::Error>,
{
    p: P,
    q: Q,
    #[new(default)]
    ph: PhantomData<I>,
}

impl<I, P, Q> Parser<I> for Then<I, P, Q>
where
    I: ?Sized + Buffer + 'static,
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
