use std::fmt::Debug;

use test_case::test_case;

use crate::parsing::Buffer;
use crate::primitive::remaining;
use crate::testutils::test_buffer_windows_output_no_res;
use crate::{Literal, Parser};

#[test_case("hello world!")]
#[test_case(b"hello world!".as_slice())]
fn remaining_then_remaining<I>(input: &I) -> anyhow::Result<()>
where
    I: ?Sized + AsRef<[u8]> + Buffer + Debug + PartialEq + 'static,
{
    test_buffer_windows_output_no_res(remaining().then(remaining()), input, |(first, second)| {
        assert_eq!(first.as_slice(), input.as_ref());
        assert!(second.is_empty());
    })
}

// #[test_case("hello ", "world!", "hello world!")]
#[test_case(b"hello ", b"world!", b"hello world!")]
fn a_then_b_buffer_windows<S, I>(a: S, b: S, input: &I) -> anyhow::Result<()>
where
    S: Literal<[u8]> + Copy + PartialEq + Debug,
    S::Error: std::error::Error + Send + Sync + 'static,
    I: ?Sized + AsRef<[u8]>,
{
    test_buffer_windows_output_no_res(a.then(b), input.as_ref(), |(aval, bval)| {
        assert_eq!(a, aval);
        assert_eq!(b, bval);
    })
}

#[test_case("hello ", "world!", "hello world!")]
#[test_case(b"hello ", b"world!", b"hello world!")]
fn a_then_b<S, I>(a: S, b: S, input: &I) -> anyhow::Result<()>
where
    S: Literal<I> + Copy + PartialEq + Debug,
    S::Error: std::error::Error + Send + Sync + 'static,
    I: ?Sized + Buffer + 'static,
{
    let (aval, bval) = a.then(b).parse_all(input)?;
    assert_eq!(a, aval);
    assert_eq!(b, bval);
    Ok(())
}
