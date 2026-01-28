use crate::{Parser, ParserCompose};

use super::{Literal, LiteralParser};

impl Literal<str> for char {
    fn literal_len(self) -> usize {
        self.len_utf8()
    }

    fn literal_eq(self, candidate: &str) -> bool {
        candidate.starts_with(self)
    }
}

impl ParserCompose for char {
}

impl Parser for char {
    type State = LiteralParser<str, char>;

    fn start_parser(self) -> Self::State {
        LiteralParser::new(self)
    }
}
