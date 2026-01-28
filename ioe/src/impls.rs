use crate::{IncpaIOE, UniversalParserError};

// Implement IncpaIOE for basic literal types

impl IncpaIOE for char {
    type Input = str;
    type Output = char;
    type Error = UniversalParserError;
}

impl IncpaIOE for &str {
    type Input = str;
    type Output = Self;
    type Error = UniversalParserError;
}

impl<T> IncpaIOE for &[T]
where
    T: PartialEq,
{
    type Input = [T];
    type Output = Self;
    type Error = UniversalParserError;
}

impl<T, const K: usize> IncpaIOE for &[T; K]
where
    T: PartialEq,
{
    type Input = [T];
    type Output = Self;
    type Error = UniversalParserError;
}
