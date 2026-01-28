use crate::{Input, UniversalParserError};

/// The [Input], `Output`, and `Error` associated with a parser
pub trait IncpaIOE: Sized {
    /// The type of input consumed
    type Input: ?Sized + Input;

    /// The type of output on successful parse
    type Output;

    /// The type of errors this parser detects
    type Error: From<UniversalParserError>;
}
