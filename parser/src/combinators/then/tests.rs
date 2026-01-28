use std::fmt::Debug;

use incpa_ioe::Input;
use test_case::test_case;

use crate::primitive::remaining;
use crate::{Literal, Parser, ParserCompose as _};

#[test_case("hello world!" => ("hello world!".to_string(), "".to_string()))]
#[test_case(b"hello world!".as_slice() => (Vec::from(b"hello world!"), vec![]))]
fn remaining_then_remaining<I>(input: &I) -> (I::Owned, I::Owned)
where
    I: ?Sized + Input + Debug + PartialEq + ToOwned + 'static,
{
    remaining().then(remaining()).parse_all(input).unwrap()
}

#[test_case("hello ", "world!", "hello world!" => ("hello ", "world!"))]
#[test_case("hello ", "world!", "hello world! SUFFIX" => ("hello ", "world!"))]
#[test_case(b"hello ", b"world!", b"hello world!" => (b"hello ", b"world!"))]
// Note: char parser only works with str input now, not [u8]
// #[test_case(b"hell", '0', b"hell0 world!" => (b"hell", '0'))]
#[test_case("hell", '0', "hell0 world!" => ("hell", '0'))]
fn a_then_b<A, B, I>(a: A, b: B, input: &I) -> (A, B)
where
    A: Literal<I> + Copy + PartialEq,
    A::Error: Debug,
    B: Literal<I, Error = A::Error> + Copy + PartialEq,
    I: ?Sized + Input + 'static,
{
    a.then(b).parse_all(input).unwrap()
}
