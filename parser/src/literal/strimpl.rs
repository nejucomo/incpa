use incpa_state::UniversalParserError;

use crate::literal::state::LiteralState;
use crate::{Literal, Parser};

impl Literal<str> for &str {
    fn literal_len(self) -> usize {
        self.len()
    }

    fn literal_eq(self, candidate: &str) -> bool {
        self == candidate
    }
}

impl Parser<str> for &str {
    type State = LiteralState<str, Self>;
    type Output = Self;
    type Error = UniversalParserError;

    fn start_parser(self) -> Self::State {
        LiteralState::new(self)
    }
}
