use derive_new::new;
use incpa_ioe::IncpaIOE;

use crate::ParserCompose;

/// Parse `P` or if that fails, parse `Q`
///
/// This holds all input while parsing `P`.
#[derive(Copy, Clone, Debug, new)]
pub struct Or<P, Q> {
    /// The primary parser, which is attempted first
    pub p: P,
    /// The alternative parser
    pub q: Q,
}

impl<P, Q> IncpaIOE for Or<P, Q>
where
    P: IncpaIOE,
    Q: IncpaIOE<Input = P::Input, Output = P::Output, Error = P::Error>,
{
    type Input = P::Input;
    type Output = P::Output;
    type Error = P::Error;
}

impl<P, Q> ParserCompose for Or<P, Q>
where
    P: ParserCompose,
    Q: ParserCompose<Input = P::Input, Output = P::Output, Error = P::Error>,
{
}
