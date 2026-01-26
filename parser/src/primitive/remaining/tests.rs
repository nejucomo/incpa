use std::fmt::Debug;

use incpa_state::Input;
use test_case::test_case;

use crate::Parser as _;
use crate::primitive::remaining;

#[test_case("hello world!")]
#[test_case(b"hello world!".as_slice())]
fn test_remaining<I>(input: &I)
where
    I: ?Sized + ToOwned + Input + Debug + PartialEq + 'static,
    I::Owned: Debug + PartialEq,
{
    assert_eq!(remaining().parse_all(input).unwrap(), input.to_owned())
}
