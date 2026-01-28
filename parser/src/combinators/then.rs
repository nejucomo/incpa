#[cfg(test)]
mod tests;

mod parser;

pub use self::parser::ThenParser;

use derive_new::new;
use incpa_ioe::IncpaIOE;

use crate::{Parser, ParserCompose};

/// Parse `P` then `Q`
#[derive(Copy, Clone, Debug, new)]
#[new(visibility = "pub(crate)")]
pub struct Then<P, Q> {
    p: P,
    q: Q,
}

impl<P, Q> IncpaIOE for Then<P, Q>
where
    P: IncpaIOE,
    Q: IncpaIOE<Input = P::Input, Error = P::Error>,
    P::Input: 'static,
{
    type Input = P::Input;
    type Output = (P::Output, Q::Output);
    type Error = P::Error;
}

impl<P, Q> ParserCompose for Then<P, Q>
where
    P: ParserCompose,
    Q: ParserCompose<Input = P::Input, Error = P::Error>,
    P::Input: 'static,
{
}

impl<P, Q> Parser for Then<P, Q>
where
    P: Parser,
    Q: Parser<Input = P::Input, Error = P::Error>,
    P::Input: 'static,
{
    type State = ThenParser<P::State, P::Output, Q::State>;

    fn start_parser(self) -> Self::State {
        ThenParser::new(self.p.start_parser(), self.q.start_parser())
    }
}
