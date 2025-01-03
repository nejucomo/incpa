use crate::syntax::literal::LiteralParser;
use crate::syntax::Literal;
use crate::{BaseParserError, Syntax};

impl<E> Literal<str, E> for char
where
    E: From<BaseParserError>,
{
    fn literal_len(self) -> usize {
        self.len_utf8()
    }

    fn literal_eq(self, candidate: &str) -> bool {
        candidate.starts_with(self)
    }
}

impl<E> Literal<[u8], E> for char
where
    E: From<BaseParserError>,
{
    fn literal_len(self) -> usize {
        self.len_utf8()
    }

    fn literal_eq(self, candidate: &[u8]) -> bool {
        let mut buf = [0; 4];
        let selfbytes = self.encode_utf8(&mut buf).as_bytes();
        selfbytes == candidate
    }
}

impl<E> Syntax<str, char, E> for char
where
    E: From<BaseParserError>,
{
    type State = LiteralParser<char>;

    fn into_parser(self) -> Self::State {
        LiteralParser::new(self)
    }
}

impl<E> Syntax<[u8], char, E> for char
where
    E: From<BaseParserError>,
{
    type State = LiteralParser<char>;

    fn into_parser(self) -> Self::State {
        LiteralParser::new(self)
    }
}
