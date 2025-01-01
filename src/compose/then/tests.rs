use std::fmt::Debug;

use test_case::test_case;

use crate::primitive::remaining;
use crate::testutils::test_buffer_windows_output_no_res;
use crate::{Buffer, Parser};

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
