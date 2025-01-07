//! Common parser testing utilities

use crate::syntax::ByteFormat;
use crate::BaseParserError;

/// Run the given parser with many different initial buffer sizes, calling `check` on the results each time
///
/// This test function helps catch bounds errors in `Parser` / `BufferManager` implementations
pub fn test_buffer_windows_res<B, I, O, E, F>(bfmt: B, input: I, check: F) -> Result<(), E>
where
    B: Clone + ByteFormat<O, E>,
    I: AsRef<[u8]>,
    E: From<BaseParserError> + From<std::io::Error>,
    F: Fn(Result<O, E>) -> Result<(), E>,
{
    for initsize in [0, 1, 2, 3, 5, 7, 1 << 10, 1 << 14] {
        eprintln!("Checking parser.parse_reader_with_initial_buffer_size(..., {initsize})");

        let res = bfmt
            .clone()
            .parse_reader_with_initial_buffer_size(input.as_ref(), initsize);

        check(res)?;
    }

    eprintln!("Checking parser.parse(...)");
    check(bfmt.parse_all(input.as_ref()))
}

/// Run the given parser with many different initial buffer sizes, calling `check` on the outputs each time
///
/// This test function helps catch bounds errors in `Parser` / `BufferManager` implementations
pub fn test_buffer_windows_outputs<B, I, O, E, F>(bfmt: B, input: I, check: F) -> Result<(), E>
where
    B: Clone + ByteFormat<O, E>,
    I: AsRef<[u8]>,
    E: From<BaseParserError> + From<std::io::Error>,
    F: Fn(O) -> Result<(), E>,
{
    test_buffer_windows_res(bfmt, input, |res| res.and_then(&check))
}

/// Run the given parser with many different initial buffer sizes, calling `check` on the outputs each time, which must panic to cause a test failure
///
/// This test function helps catch bounds errors in `Parser` / `BufferManager` implementations
pub fn test_buffer_windows_output_no_res<B, I, O, E, F>(
    bfmt: B,
    input: I,
    check: F,
) -> Result<(), E>
where
    B: Clone + ByteFormat<O, E>,
    I: AsRef<[u8]>,
    E: From<BaseParserError> + From<std::io::Error>,
    F: Fn(O),
{
    test_buffer_windows_res(bfmt, input, |res| {
        let output = res?;
        check(output);
        Ok(())
    })
}
