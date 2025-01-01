use std::fmt::Debug;

use test_case::test_case;

use crate::primitive::remaining;
use crate::testutils::test_buffer_windows_output_no_res;
use crate::Buffer;

#[test_case("hello world!")]
#[test_case(b"hello world!".as_slice())]
fn test_remaining<I>(input: &I) -> Result<(), anyhow::Error>
where
    I: ?Sized + AsRef<[u8]> + Buffer + Debug + PartialEq + 'static,
{
    test_buffer_windows_output_no_res(remaining(), input, |output| {
        assert_eq!(output.as_slice(), input.as_ref());
    })
}
