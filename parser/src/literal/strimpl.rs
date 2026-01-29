use crate::{Literal, Parser};

use super::LiteralState;

impl Literal<str> for &str {
    fn literal_len(self) -> usize {
        self.len()
    }

    fn literal_eq(self, candidate: &str) -> bool {
        self == candidate
    }
}

impl<'a> Parser for &'a str {
    type State = LiteralState<str, &'a str>;

    fn start_parser(self) -> Self::State {
        LiteralState::new(self)
    }
}
