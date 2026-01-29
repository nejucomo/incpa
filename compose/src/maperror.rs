use std::marker::PhantomData;

use derive_new::new;
use incpa_ioe::{IncpaIOE, UniversalParserError};

use crate::ParserCompose;

/// Specifies a parser which maps its error
#[derive(Copy, Clone, Debug, new)]
pub struct MapError<P, F, E> {
    /// The inner parser or state
    pub inner: P,
    /// The error mapping function
    pub f: F,
    #[new(default)]
    ph: PhantomData<E>,
}

impl<P, F, E> IncpaIOE for MapError<P, F, E>
where
    P: IncpaIOE,
    F: FnOnce(P::Error) -> E,
    E: From<UniversalParserError>,
{
    type Input = P::Input;
    type Output = P::Output;
    type Error = E;
}

impl<P, F, E> ParserCompose for MapError<P, F, E>
where
    P: ParserCompose,
    F: FnOnce(P::Error) -> E,
    E: From<UniversalParserError>,
{
}
