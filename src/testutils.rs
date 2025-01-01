//! Common parser testing utilities

use crate::{ByteParser, Error};

/// Run the given parser with many different initial buffer sizes, calling `check` on the results each time
///
/// This test function helps catch bounds errors in `Parser` / `BufferManager` implementations
pub fn test_buffer_windows_res<P, I, O, E, F>(parser: P, input: I, check: F) -> Result<(), E>
where
    P: Clone + ByteParser<O, E>,
    I: AsRef<[u8]>,
    E: From<Error> + From<std::io::Error>,
    F: Fn(Result<O, E>) -> Result<(), E>,
{
    for initsize in [0, 1, 2, 3, 5, 7, 1 << 10, 1 << 14] {
        eprintln!("Checking parser.parse_reader_with_initial_buffer_size(..., {initsize}...");

        let res = parser
            .clone()
            .parse_reader_with_initial_buffer_size(input.as_ref(), initsize);

        check(res)?;
    }

    eprintln!("Checking parser.parse(...)");
    check(parser.parse(input.as_ref()))
}

/// Run the given parser with many different initial buffer sizes, calling `check` on the outputs each time
///
/// This test function helps catch bounds errors in `Parser` / `BufferManager` implementations
pub fn test_buffer_windows_outputs<P, I, O, E, F>(parser: P, input: I, check: F) -> Result<(), E>
where
    P: Clone + ByteParser<O, E>,
    I: AsRef<[u8]>,
    E: From<Error> + From<std::io::Error>,
    F: Fn(O) -> Result<(), E>,
{
    test_buffer_windows_res(parser, input, |res| res.and_then(&check))
}

/// Run the given parser with many different initial buffer sizes, calling `check` on the outputs each time, which must panic to cause a test failure
///
/// This test function helps catch bounds errors in `Parser` / `BufferManager` implementations
pub fn test_buffer_windows_output_no_res<P, I, O, E, F>(
    parser: P,
    input: I,
    check: F,
) -> Result<(), E>
where
    P: Clone + ByteParser<O, E>,
    I: AsRef<[u8]>,
    E: From<Error> + From<std::io::Error>,
    F: Fn(O),
{
    test_buffer_windows_res(parser, input, |res| {
        let output = res?;
        check(output);
        Ok(())
    })
}
