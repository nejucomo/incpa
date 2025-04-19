use crate::syntax::{Literal, LiteralParser};
use crate::{BaseParserError, Parser};

impl Literal<str> for &str {
    fn literal_len(self) -> usize {
        self.len()
    }

    fn literal_eq(self, candidate: &str) -> bool {
        self == candidate
    }
}

impl<'a> Parser<str> for &'a str {
    type Output = Self;
    type Error = BaseParserError;
    type State = LiteralParser<&'a str>;

    fn into_parser(self) -> Self::State {
        LiteralParser::new(self)
    }
}
