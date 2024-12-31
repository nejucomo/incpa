/// "Universal" errors that any parser may encounter
pub enum Error {
    /// The parser expects more input by the end of input
    ExpectedMoreInput,
    /// The parser encountered unexpected input
    UnexpectedInput,
}
