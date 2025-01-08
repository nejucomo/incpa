use std::fmt::Debug;

use either::Either::{self, Left, Right};
use test_case::test_case;

use crate::parsing::Buffer;
use crate::testutils::test_buffer_windows_res;
use crate::Syntax;

// #[test_case("hello ", "world!", "hello world!")]
#[test_case(b"hello", b"world", b"hello world!", Some(Left(b"hello")))]
#[test_case(b"hello", b"world", b"world: hello!", Some(Right(b"world")))]
#[test_case(b"hello", b"world", b"smorgasbord", None)]
fn a_or_b_buffer_windows<S, I>(
    a: S,
    b: S,
    input: &I,
    expected: Option<Either<S, S>>,
) -> anyhow::Result<()>
where
    S: Syntax<[u8], S, anyhow::Error> + Copy + PartialEq + Debug,
    I: ?Sized + AsRef<[u8]>,
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
fn a_or_b<S, I>(a: S, b: S, input: &I) -> Option<Either<S, S>>
where
    S: Syntax<I, S, anyhow::Error> + Copy + PartialEq + Debug,
    I: ?Sized + Buffer + 'static,
{
    a.or(b).parse_all(input).ok()
}
