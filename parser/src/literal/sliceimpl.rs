use incpa_state::UniversalParserError;

use crate::literal::state::LiteralState;
use crate::{Literal, Parser};

impl<T> Literal<[T]> for &[T]
where
    T: PartialEq,
{
    fn literal_len(self) -> usize {
        self.len()
    }

    fn literal_eq(self, candidate: &[T]) -> bool {
        self == candidate
    }
}

impl<T> Parser<[T]> for &[T]
where
    T: PartialEq,
{
    type State = LiteralState<[T], Self>;
    type Output = Self;
    type Error = UniversalParserError;

    fn start_parser(self) -> Self::State {
        LiteralState::new(self)
    }
}
