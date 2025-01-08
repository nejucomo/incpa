use crate::syntax::{Literal, LiteralParser};
use crate::{BaseParserError, Syntax};

impl<E> Literal<str, E> for &str
where
    E: From<BaseParserError>,
{
    fn literal_len(self) -> usize {
        self.len()
    }

    fn literal_eq(self, candidate: &str) -> bool {
        self == candidate
    }
}

impl<'a, E> Syntax<str, &'a str, E> for &'a str
where
    E: From<BaseParserError>,
{
    type State = LiteralParser<&'a str>;

    fn into_parser(self) -> Self::State {
        LiteralParser::new(self)
    }
}
