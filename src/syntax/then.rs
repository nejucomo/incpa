#[cfg(test)]
mod tests;

mod parser;

pub use self::parser::ThenParser;

use derive_new::new;

use crate::parsing::Buffer;
use crate::{BaseParserError, Syntax};

/// Parse `P` then `Q`
#[derive(Copy, Clone, Debug, new)]
#[new(visibility = "pub(crate)")]
pub struct Then<P, Q> {
    p: P,
    q: Q,
}

impl<P, Q, I, PO, QO, E> Syntax<I, (PO, QO), E> for Then<P, Q>
where
    I: ?Sized + Buffer + 'static,
    P: Syntax<I, PO, E>,
    Q: Syntax<I, QO, E>,
    E: From<BaseParserError>,
{
    type State = ThenParser<P::State, PO, Q::State>;

    fn into_parser(self) -> Self::State {
        ThenParser::new(self.p.into_parser(), self.q.into_parser())
    }
}
