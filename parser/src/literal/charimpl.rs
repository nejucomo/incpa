use crate::Parser;

use super::{Literal, LiteralState};

impl Literal<str> for char {
    fn literal_len(self) -> usize {
        self.len_utf8()
    }

    fn literal_eq(self, candidate: &str) -> bool {
        candidate.starts_with(self)
    }
}

impl Parser for char {
    type State = LiteralState<str, char>;

    fn start_parser(self) -> Self::State {
        LiteralState::new(self)
    }
}
