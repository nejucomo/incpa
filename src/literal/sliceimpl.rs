use crate::{Parser, ParserCompose, ParserOutErr, UniversalParserError};

use super::{Literal, LiteralParser};

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

impl<T> ParserOutErr for &[T]
where
    T: PartialEq,
{
    type Output = Self;
    type Error = UniversalParserError;
}

impl<T> ParserCompose for &[T] where T: PartialEq {}

impl<'a, T> Parser<[T]> for &'a [T]
where
    T: PartialEq,
{
    type State = LiteralParser<&'a [T]>;

    fn start_parser(self) -> Self::State {
        LiteralParser::new(self)
    }
}
