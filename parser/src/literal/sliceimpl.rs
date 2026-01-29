use crate::Parser;

use super::{Literal, LiteralState};

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

impl<'a, T> Parser for &'a [T]
where
    T: PartialEq,
{
    type State = LiteralState<[T], &'a [T]>;

    fn start_parser(self) -> Self::State {
        LiteralState::new(self)
    }
}
