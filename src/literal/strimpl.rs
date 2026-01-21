use crate::{Literal, ParserCompose, ParserOutErr};
use crate::{Parser, UniversalParserError};

use super::LiteralParser;

impl Literal<str> for &str {
    fn literal_len(self) -> usize {
        self.len()
    }

    fn literal_eq(self, candidate: &str) -> bool {
        self == candidate
    }
}

impl ParserOutErr for &str {
    type Output = Self;
    type Error = UniversalParserError;
}

impl ParserCompose for &str {}

impl<'a> Parser<str> for &'a str {
    type State = LiteralParser<&'a str>;

    fn start_parser(self) -> Self::State {
        LiteralParser::new(self)
    }
}
