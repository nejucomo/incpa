use crate::UniversalParserError;

/// A common base trait for associated output and error types
pub trait ParserOutput {
    /// The type of output on successful parse
    type Output;

    /// The type of errors this parser detects
    type Error: From<UniversalParserError>;
}
