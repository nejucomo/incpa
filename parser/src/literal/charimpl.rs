use incpa_state::UniversalParserError;

use crate::literal::state::LiteralState;
use crate::{Literal, Parser};

impl Literal<str> for char {
    fn literal_len(self) -> usize {
        self.len_utf8()
    }

    fn literal_eq(self, candidate: &str) -> bool {
        candidate.starts_with(self)
    }
}

impl Parser<str> for char {
    type State = LiteralState<str, char>;
    type Output = Self;
    type Error = UniversalParserError;

    fn start_parser(self) -> Self::State {
        LiteralState::new(self)
    }
}
