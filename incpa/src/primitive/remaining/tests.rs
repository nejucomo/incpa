use std::fmt::Debug;

use test_case::test_case;

use crate::Parser as _;
use crate::primitive::remaining;
use crate::state::Buffer;

#[test_case("hello world!")]
#[test_case(b"hello world!".as_slice())]
fn test_remaining<I>(input: &I)
where
    I: ?Sized + ToOwned + Buffer + Debug + PartialEq + 'static,
    I::Owned: Debug + PartialEq,
{
    assert_eq!(remaining().parse_all(input).unwrap(), input.to_owned())
}
