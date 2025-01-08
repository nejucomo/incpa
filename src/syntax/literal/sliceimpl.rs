use crate::syntax::literal::LiteralParser;
use crate::syntax::Literal;
use crate::{BaseParserError, Syntax};

impl<T, E> Literal<[T], E> for &[T]
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

impl<'a, T, E> Syntax<[T], &'a [T], E> for &'a [T]
where
    T: PartialEq,
    E: From<BaseParserError>,
{
    type State = LiteralParser<&'a [T]>;

    fn into_parser(self) -> Self::State {
        LiteralParser::new(self)
    }
}
