use std::marker::PhantomData;

use derive_new::new;
use incpa_ioe::IncpaIOE;

use crate::ParserCompose;

/// Specifies a parser which maps its output
#[derive(Copy, Clone, Debug, new)]
pub struct MapOutput<P, F, O> {
    /// The inner parser or state
    pub inner: P,
    /// The output mapping function
    pub f: F,
    #[new(default)]
    ph: PhantomData<O>,
}

impl<P, F, O> IncpaIOE for MapOutput<P, F, O>
where
    P: IncpaIOE,
    F: FnOnce(P::Output) -> O,
{
    type Input = P::Input;
    type Output = O;
    type Error = P::Error;
}

impl<P, F, O> ParserCompose for MapOutput<P, F, O>
where
    P: ParserCompose,
    F: FnOnce(P::Output) -> O,
{
}
