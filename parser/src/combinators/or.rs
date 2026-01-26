#[cfg(test)]
mod tests;

mod parser;

pub use self::parser::OrParser;

use derive_new::new;
use either::Either;
use incpa_state::Input;

use crate::{Parser, ParserCompose};

/// Parse `P` or if that fails, parse `Q`
///
/// This holds all input while parsing `P`.
#[derive(Copy, Clone, Debug, new)]
#[new(visibility = "pub(crate)")]
pub struct Or<P, Q> {
    p: P,
    q: Q,
}

impl<P, Q> ParserCompose for Or<P, Q>
where
    P: ParserCompose,
    Q: ParserCompose<Error = P::Error>,
{
    type Output = Either<P::Output, Q::Output>;
    type Error = P::Error;
}

impl<P, Q, I> Parser<I> for Or<P, Q>
where
    I: ?Sized + Input + 'static,
    P: Parser<I>,
    Q: Parser<I, Error = P::Error>,
{
    type State = OrParser<P::State, Q::State>;

    fn start_parser(self) -> Self::State {
        OrParser::new(self.p.start_parser(), self.q.start_parser())
    }
}
