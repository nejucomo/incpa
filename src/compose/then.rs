#[cfg(test)]
mod tests;

mod state;

pub use self::state::ThenState;

use std::marker::PhantomData;

use derive_new::new;

use crate::parsing::Buffer;
use crate::{BaseParserError, Syntax};

/// Parses `P` then `Q`
#[derive(Copy, Clone, Debug, new)]
#[new(visibility = "pub(crate)")]
pub struct Then<P, O, Q> {
    p: P,
    q: Q,
    #[new(default)]
    ph: PhantomData<O>,
}

impl<P, Q, I, PO, QO, E> Syntax<I, (PO, QO), E> for Then<P, PO, Q>
where
    I: ?Sized + Buffer + 'static,
    P: Syntax<I, PO, E>,
    Q: Syntax<I, QO, E>,
    E: From<BaseParserError>,
{
    type State = ThenState<P::State, PO, Q::State>;

    fn into_parser(self) -> Self::State {
        ThenState::new(self.p.into_parser(), self.q.into_parser())
    }
}
