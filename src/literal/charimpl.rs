use crate::{BaseParserError, Parser};

use super::{Literal, LiteralParser};

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

impl Parser<str> for char {
    type Output = char;
    type Error = BaseParserError;
    type State = LiteralParser<char>;

    fn into_parser(self) -> Self::State {
        LiteralParser::new(self)
    }
}

impl Parser<[u8]> for char {
    type Output = char;
    type Error = BaseParserError;
    type State = LiteralParser<char>;

    fn into_parser(self) -> Self::State {
        LiteralParser::new(self)
    }
}
