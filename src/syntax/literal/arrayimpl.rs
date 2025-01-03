use crate::syntax::literal::LiteralParser;
use crate::syntax::Literal;
use crate::{BaseParserError, Syntax};

impl<T, const K: usize, E> Literal<[T], E> for &[T; K]
where
    T: PartialEq,
    E: From<BaseParserError>,
{
    fn literal_len(self) -> usize {
        self.len()
    }

    fn literal_eq(self, candidate: &[T]) -> bool {
        self == candidate
    }
}

impl<'a, T, const K: usize, E> Syntax<[T], &'a [T; K], E> for &'a [T; K]
where
    T: PartialEq,
    E: From<BaseParserError>,
{
    type State = LiteralParser<&'a [T; K]>;

    fn into_parser(self) -> Self::State {
        LiteralParser::new(self)
    }
}
