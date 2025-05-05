use crate::{Parser, ParserCombinator, ParserOutput, UniversalParserError};

use super::{Literal, LiteralParserState};

impl<T, const K: usize> Literal<[T]> for &[T; K]
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

impl<T, const K: usize> ParserOutput for &[T; K]
where
    T: PartialEq,
{
    type Output = Self;
    type Error = UniversalParserError;
}

impl<T, const K: usize> ParserCombinator for &[T; K] where T: PartialEq {}

impl<T, const K: usize> Parser<[T]> for &[T; K]
where
    T: PartialEq,
{
    type State = LiteralParserState<Self>;

    fn start_parser(self) -> Self::State {
        LiteralParserState::new(self)
    }
}
