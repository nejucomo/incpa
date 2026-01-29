use crate::Parser;

use super::{Literal, LiteralState};

impl<T, const K: usize> Literal<[T]> for &[T; K]
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

impl<'a, T, const K: usize> Parser for &'a [T; K]
where
    T: PartialEq,
{
    type State = LiteralState<[T], &'a [T; K]>;

    fn start_parser(self) -> Self::State {
        LiteralState::new(self)
    }
}
