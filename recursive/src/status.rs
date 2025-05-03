/// The status from [RecursiveParserState::feed_recursive](crate::RecursiveParserState::feed_recursive)
#[derive(Debug)]
pub enum RecursionStatus<S, C> {
    /// A [RecursiveParserState](crate::RecursiveParserState)
    RPS(S),

    /// A continuation
    ContinueWith(C),
}
