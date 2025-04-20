use std::fmt::Debug;

use either::Either::{self, Left, Right};
use test_case::test_case;

use crate::state::Buffer;
use crate::{Literal, Parser};

#[test_case("hello", "world", "hello world!" => Some(Left("hello")))]
#[test_case("hello", "world", "world: hello!" => Some(Right("world")))]
#[test_case("hello", "world", "smorgasbord" => None)]
#[test_case(b"hello", b"world", b"hello world!" => Some(Left(b"hello")))]
#[test_case(b"hello", b"world", b"world: hello!" => Some(Right(b"world")))]
#[test_case(b"hello", b"world", b"smorgasbord" => None)]
#[test_case('X', b"world", b"X!" => Some(Left('X')))]
#[test_case('X', b"world", b"world" => Some(Right(b"world")))]
#[test_case('X', b"world", b"smorgasbord" => None)]
fn a_or_b<A, B, I>(a: A, b: B, input: &I) -> Option<Either<A, B>>
where
    A: Literal<I> + Copy + PartialEq + Debug,
    B: Literal<I, Error = A::Error> + Copy + PartialEq + Debug,
    I: ?Sized + Buffer + 'static,
{
    a.or(b).parse_all(input).ok()
}
