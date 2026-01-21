use crate::{Parser, ParserCompose, UniversalParserError};

use super::{Literal, LiteralParser};

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

impl<T, const K: usize> ParserCompose for &[T; K]
where
    T: PartialEq,
{
    type Output = Self;
    type Error = UniversalParserError;
}

impl<'a, T, const K: usize> Parser<[T]> for &'a [T; K]
where
    T: PartialEq,
{
    type State = LiteralParser<&'a [T; K]>;

    fn start_parser(self) -> Self::State {
        LiteralParser::new(self)
    }
}
