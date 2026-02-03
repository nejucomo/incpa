use std::fmt::Debug;

use incpa_state::Input;
use test_case::test_case;

use crate::compose::ParserCompose as _;
use crate::{Literal, Parser};

#[test_case("hello", "world", "hello world!" => Some("hello"))]
#[test_case("hello", "world", "world: hello!" => Some("world"))]
#[test_case("hello", "world", "smorgasbord" => None)]
#[test_case(b"hello", b"world", b"hello world!" => Some(b"hello"))]
#[test_case(b"hello", b"world", b"world: hello!" => Some(b"world"))]
#[test_case(b"hello", b"world", b"smorgasbord" => None)]
// Note: char parser only works with str input now, not [u8]
// #[test_case('X', b"world", b"X!" => Some('X'))]
// #[test_case('X', b"world", b"world" => Some(b"world"))]
// #[test_case('X', b"world", b"smorgasbord" => None)]
fn a_or_b<L, I>(a: L, b: L, input: &I) -> Option<L>
where
    L: Literal<I> + Copy + PartialEq + Debug,
    I: ?Sized + Input + 'static,
{
    a.or(b).parse_all(input).ok()
}
