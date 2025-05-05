use crate::{Parser, ParserCombinator, ParserOutput, UniversalParserError};

use super::{Literal, LiteralParserState};

impl<T> Literal<[T]> for &[T]
where
    T: PartialEq,
{
    fn literal_len(self) -> usize {
        self.len()
    }

    fn literal_eq(self, candidate: &[T]) -> bool {
        self == candidate
    }
}

impl<T> ParserOutput for &[T]
where
    T: PartialEq,
{
    type Output = Self;
    type Error = UniversalParserError;
}

impl<T> ParserCombinator for &[T] where T: PartialEq {}

impl<T> Parser<[T]> for &[T]
where
    T: PartialEq,
{
    type State = LiteralParserState<Self>;

    fn start_parser(self) -> Self::State {
        LiteralParserState::new(self)
    }
}
