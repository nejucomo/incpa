use std::fmt::Debug;

use test_case::test_case;

use crate::primitive::remaining;
use crate::state::Buffer;
use crate::{Literal, Parser};

#[test_case("hello world!" => ("hello world!".to_string(), "".to_string()))]
#[test_case(b"hello world!".as_slice() => (Vec::from(b"hello world!"), vec![]))]
fn remaining_then_remaining<I>(input: &I) -> (I::Owned, I::Owned)
where
    I: ?Sized + Buffer + Debug + PartialEq + ToOwned + 'static,
{
    remaining().then(remaining()).parse_all(input).unwrap()
}

#[test_case("hello ", "world!", "hello world!" => ("hello ", "world!"))]
#[test_case("hello ", "world!", "hello world! SUFFIX" => ("hello ", "world!"))]
#[test_case(b"hello ", b"world!", b"hello world!" => (b"hello ", b"world!"))]
#[test_case(b"hell", '0', b"hell0 world!" => (b"hell", '0'))]
#[test_case("hell", '0', "hell0 world!" => ("hell", '0'))]
fn a_then_b<A, B, I>(a: A, b: B, input: &I) -> (A, B)
where
    A: Literal<I> + Copy + PartialEq,
    A::Error: Debug,
    B: Literal<I, Error = A::Error> + Copy + PartialEq,
    I: ?Sized + Buffer + 'static,
{
    a.then(b).parse_all(input).unwrap()
}
