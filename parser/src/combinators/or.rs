#[cfg(test)]
mod tests;

mod parser;

pub use self::parser::OrParser;

use derive_new::new;
use either::Either;
use incpa_ioe::IncpaIOE;

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

impl<P, Q> IncpaIOE for Or<P, Q>
where
    P: IncpaIOE,
    Q: IncpaIOE<Input = P::Input, Error = P::Error>,
{
    type Input = P::Input;
    type Output = Either<P::Output, Q::Output>;
    type Error = P::Error;
}

impl<P, Q> ParserCompose for Or<P, Q>
where
    P: ParserCompose,
    Q: ParserCompose<Input = P::Input, Error = P::Error>,
{
}

impl<P, Q> Parser for Or<P, Q>
where
    P: Parser,
    Q: Parser<Input = P::Input, Error = P::Error>,
{
    type State = OrParser<P::State, Q::State>;

    fn start_parser(self) -> Self::State {
        OrParser::new(self.p.start_parser(), self.q.start_parser())
    }
}
