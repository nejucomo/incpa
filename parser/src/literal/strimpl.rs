use crate::{Literal, Parser, ParserCompose};

use super::LiteralParser;

impl Literal<str> for &str {
    fn literal_len(self) -> usize {
        self.len()
    }

    fn literal_eq(self, candidate: &str) -> bool {
        self == candidate
    }
}

impl ParserCompose for &str {
}

impl<'a> Parser for &'a str {
    type State = LiteralParser<str, &'a str>;

    fn start_parser(self) -> Self::State {
        LiteralParser::new(self)
    }
}
