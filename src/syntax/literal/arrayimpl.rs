use crate::syntax::literal::LiteralParser;
use crate::syntax::Literal;
use crate::{BaseParserError, Syntax};

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

impl<'a, T, const K: usize> Syntax<[T]> for &'a [T; K]
where
    T: PartialEq,
{
    type Output = Self;
    type Error = BaseParserError;
    type State = LiteralParser<&'a [T; K]>;

    fn into_parser(self) -> Self::State {
        LiteralParser::new(self)
    }
}
