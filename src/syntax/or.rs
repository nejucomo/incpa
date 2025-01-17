#[cfg(test)]
mod tests;

mod parser;

pub use self::parser::OrParser;

use derive_new::new;
use either::Either;

use crate::parsing::Buffer;
use crate::Syntax;

/// Parse `P` or if that fails, parse `Q`
///
/// This holds all input while parsing `P`.
#[derive(Copy, Clone, Debug, new)]
#[new(visibility = "pub(crate)")]
pub struct Or<P, Q> {
    p: P,
    q: Q,
}

impl<P, Q, I> Syntax<I> for Or<P, Q>
where
    I: ?Sized + Buffer + 'static,
    P: Syntax<I>,
    Q: Syntax<I, Error = P::Error>,
{
    type Output = Either<P::Output, Q::Output>;
    type Error = P::Error;
    type State = OrParser<P::State, Q::State>;

    fn into_parser(self) -> Self::State {
        OrParser::new(self.p.into_parser(), self.q.into_parser())
    }
}
