//! Common parser testing utilities

use crate::{BaseParserError, ByteParser, Parser, Syntax};

/// Run the given parser with many different initial buffer sizes, calling `check` on the results each time
///
/// This test function helps catch bounds errors in `Parser` / `BufferManager` implementations
pub fn test_buffer_windows_res<P, I, O, E, F>(pspec: P, input: I, check: F) -> Result<(), E>
where
    P: Clone + Syntax<[u8], O, E>,
    I: AsRef<[u8]>,
    E: From<BaseParserError> + From<std::io::Error>,
    F: Fn(Result<O, E>) -> Result<(), E>,
{
    for initsize in [0, 1, 2, 3, 5, 7, 1 << 10, 1 << 14] {
        eprintln!("Checking parser.parse_reader_with_initial_buffer_size(..., {initsize})");

        let res = pspec
            .clone()
            .into_parser()
            .parse_reader_with_initial_buffer_size(input.as_ref(), initsize);

        check(res)?;
    }

    eprintln!("Checking parser.parse(...)");
    check(pspec.into_parser().parse(input.as_ref()))
}

/// Run the given parser with many different initial buffer sizes, calling `check` on the outputs each time
///
/// This test function helps catch bounds errors in `Parser` / `BufferManager` implementations
pub fn test_buffer_windows_outputs<P, I, O, E, F>(pspec: P, input: I, check: F) -> Result<(), E>
where
    P: Clone + Syntax<[u8], O, E>,
    I: AsRef<[u8]>,
    E: From<BaseParserError> + From<std::io::Error>,
    F: Fn(O) -> Result<(), E>,
{
    test_buffer_windows_res(pspec, input, |res| res.and_then(&check))
}

/// Run the given parser with many different initial buffer sizes, calling `check` on the outputs each time, which must panic to cause a test failure
///
/// This test function helps catch bounds errors in `Parser` / `BufferManager` implementations
pub fn test_buffer_windows_output_no_res<P, I, O, E, F>(
    pspec: P,
    input: I,
    check: F,
) -> Result<(), E>
where
    P: Clone + Syntax<[u8], O, E>,
    I: AsRef<[u8]>,
    E: From<BaseParserError> + From<std::io::Error>,
    F: Fn(O),
{
    test_buffer_windows_res(pspec, input, |res| {
        let output = res?;
        check(output);
        Ok(())
    })
}
