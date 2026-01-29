use derive_new::new;
use incpa_ioe::IncpaIOE;

use crate::ParserCompose;

/// Parse `P` then `Q`
#[derive(Copy, Clone, Debug, new)]
#[new(visibility = "pub(crate)")]
pub struct Then<P, Q> {
    /// The initial parser
    pub p: P,
    /// The subsequent parser
    pub q: Q,
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
