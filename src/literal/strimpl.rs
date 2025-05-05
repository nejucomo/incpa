use crate::{Literal, ParserCombinator, ParserOutput};
use crate::{Parser, UniversalParserError};

use super::LiteralParserState;

impl Literal<str> for &str {
    fn literal_len(self) -> usize {
        self.len()
    }

    fn literal_eq(self, candidate: &str) -> bool {
        self == candidate
    }
}

impl ParserOutput for &str {
    type Output = Self;
    type Error = UniversalParserError;
}

impl ParserCombinator for &str {}

impl Parser<str> for &str {
    type State = LiteralParserState<Self>;

    fn start_parser(self) -> Self::State {
        LiteralParserState::new(self)
    }
}
