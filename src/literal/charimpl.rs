use crate::{Parser, ParserCombinator, ParserOutput, UniversalParserError};

use super::{Literal, LiteralParserState};

impl Literal<str> for char {
    fn literal_len(self) -> usize {
        self.len_utf8()
    }

    fn literal_eq(self, candidate: &str) -> bool {
        candidate.starts_with(self)
    }
}

impl Literal<[u8]> for char {
    fn literal_len(self) -> usize {
        self.len_utf8()
    }

    fn literal_eq(self, candidate: &[u8]) -> bool {
        let mut buf = [0; 4];
        let selfbytes = self.encode_utf8(&mut buf).as_bytes();
        selfbytes == candidate
    }
}

impl ParserOutput for char {
    type Output = char;
    type Error = UniversalParserError;
}

impl ParserCombinator for char {}

impl Parser<str> for char {
    type State = LiteralParserState<char>;

    fn start_parser(self) -> Self::State {
        LiteralParserState::new(self)
    }
}

impl Parser<[u8]> for char {
    type State = LiteralParserState<char>;

    fn start_parser(self) -> Self::State {
        LiteralParserState::new(self)
    }
}
