use crate::syntax::literal::LiteralParser;
use crate::syntax::Literal;
use crate::{BaseParserError, Syntax};

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

impl<'a, T> Syntax<[T]> for &'a [T]
where
    T: PartialEq,
{
    type Output = Self;
    type Error = BaseParserError;
    type State = LiteralParser<&'a [T]>;

    fn into_parser(self) -> Self::State {
        LiteralParser::new(self)
    }
}
