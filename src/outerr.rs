use crate::UniversalParserError;

/// A base trait for all types with a defined output and error
pub trait ParserOutErr: Sized {
    /// The type of output on successful parse
    type Output;

    /// The type of errors this parser detects
    type Error: From<UniversalParserError>;
}
