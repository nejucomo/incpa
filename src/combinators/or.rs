#[cfg(test)]
mod tests;

mod parser;

use std::marker::PhantomData;

pub use self::parser::OrParser;

use derive_new::new;
use either::Either;

use crate::Parser;
use crate::state::Buffer;

/// Parse `P` or if that fails, parse `Q`
///
/// This holds all input while parsing `P`.
#[derive(Copy, Clone, Debug, new)]
#[new(visibility = "pub(crate)")]
pub struct Or<I, P, Q>
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

impl<I, P, Q> Parser<I> for Or<I, P, Q>
where
    I: ?Sized + Buffer + 'static,
    P: Parser<I>,
    Q: Parser<I, Error = P::Error>,
{
    type Output = Either<P::Output, Q::Output>;
    type Error = P::Error;
    type State = OrParser<P::State, Q::State>;

    fn start_parser(self) -> Self::State {
        OrParser::new(self.p.start_parser(), self.q.start_parser())
    }
}
