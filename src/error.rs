/// Universal errors that any parser may encounter
#[derive(Copy, Clone, Debug, thiserror::Error, Eq, PartialEq)]
pub enum UniversalParserError {
    /// The parser expects more input by the end of input
    #[error("expected more input")]
    ExpectedMoreInput,
    /// The parser encountered unexpected input
    #[error("encountered unexpected input")]
    UnexpectedInput,
}
