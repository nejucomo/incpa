//! UTF8 support

use derive_more::From;
use incpa::state::{ChompedExt as _, FeedChomped, OutcomeExt as _, ParserState};
use incpa::{Parser, UniversalParserError};
use thiserror::Error;

use crate::StrParser;

/// Provide a byte-input parser for an inner str parser
#[derive(Debug, From)]
pub struct Utf8Adapter<P>(P)
where
    P: StrParser;

/// A byte-input [ParserState] for an inner str parser
#[derive(Debug, From)]
pub struct Utf8AdapterState<S>(S)
where
    S: ParserState<str>;

/// An error from either the StrParser or malformed utf8
#[derive(Debug, Error)]
pub enum Utf8AdapterError<E> {
    /// An error with utf8 decoding
    #[error(transparent)]
    Utf8(#[from] std::str::Utf8Error),
    /// An error from the inner [StrParser]
    #[error(transparent)]
    StrParser(E),
}

impl<P> Parser<[u8]> for Utf8Adapter<P>
where
    P: StrParser,
    P::Error: From<UniversalParserError>,
{
    type Output = P::Output;
    type Error = Utf8AdapterError<P::Error>;
    type State = Utf8AdapterState<P::State>;

    fn start_parser(self) -> Self::State {
        Utf8AdapterState::from(self.0.start_parser())
    }
}

impl<S> ParserState<[u8]> for Utf8AdapterState<S>
where
    S: ParserState<str>,
{
    type Output = S::Output;
    type Error = Utf8AdapterError<S::Error>;

    fn feed(self, input: &[u8]) -> Result<FeedChomped<Self, Self::Output>, Self::Error> {
        let s = std::str::from_utf8(input)?;
        let update = self.0.feed(s).map_err(Utf8AdapterError::StrParser)?;
        Ok(update
            .map_parser(Utf8AdapterState)
            .map_consumed(|c| s.char_indices().nth(c).map(|(b, _)| b).unwrap_or(s.len())))
    }
}

impl<E> From<UniversalParserError> for Utf8AdapterError<E>
where
    E: From<UniversalParserError>,
{
    fn from(bpe: UniversalParserError) -> Self {
        Utf8AdapterError::StrParser(E::from(bpe))
    }
}
