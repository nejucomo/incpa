use std::fmt::Debug;

use either::Either::{self, Left, Right};
use test_case::test_case;

use crate::parsing::Buffer;
use crate::syntax::Literal;
use crate::testutils::test_buffer_windows_res;
use crate::Parser;

#[test_case(b"hello", b"world", b"hello world!", Some(Left(b"hello")))]
#[test_case(b"hello", b"world", b"world: hello!", Some(Right(b"world")))]
#[test_case(b"hello", b"world", b"smorgasbord", None)]
fn a_or_b_buffer_windows<A, B, I>(
    a: A,
    b: B,
    input: I,
    expected: Option<Either<A, B>>,
) -> anyhow::Result<()>
where
    A: Literal<[u8]> + PartialEq + Debug,
    A::Error: std::error::Error + Send + Sync + 'static,
    B: Literal<[u8], Error = A::Error> + PartialEq + Debug,
    I: AsRef<[u8]>,
{
    test_buffer_windows_res(a.or(b), input.as_ref(), |output| {
        assert_eq!(output.ok(), expected);
        Ok(())
    })
}

#[test_case("hello", "world", "hello world!" => Some(Left("hello")))]
#[test_case("hello", "world", "world: hello!" => Some(Right("world")))]
#[test_case("hello", "world", "smorgasbord" => None)]
#[test_case(b"hello", b"world", b"hello world!" => Some(Left(b"hello")))]
#[test_case(b"hello", b"world", b"world: hello!" => Some(Right(b"world")))]
#[test_case(b"hello", b"world", b"smorgasbord" => None)]
fn a_or_b<L, I>(a: L, b: L, input: &I) -> Option<Either<L, L>>
where
    L: Literal<I> + Copy + PartialEq + Debug,
    I: ?Sized + Buffer + 'static,
{
    a.or(b).parse_all(input).ok()
}
