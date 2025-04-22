use incpa::Parser;

use crate::utf8::Utf8Adapter;

impl<P> StrParser for P where P: Parser<str> {}

/// A parser of `&str` input
pub trait StrParser: Parser<str> {
    /// Convert into a `Parser<[u8]>` that expects UTF8 byte input.
    ///
    /// See `incpa-byte` for a convenient way to read `std::io::Read` inputs, and `incpa-tokio` for a convenient way to read `tokio::io::AsyncRead` inputs.
    fn into_utf8_adapter(self) -> Utf8Adapter<Self> {
        Utf8Adapter::from(self)
    }
}

#[test]
fn test_hello_world_via_adapter() {
    use incpa::Parser;

    let strp = "Hello World!";
    let parsed = strp
        .into_utf8_adapter()
        .parse_all(b"Hello World!".as_slice())
        .unwrap();

    assert_eq!(strp, parsed);
}
