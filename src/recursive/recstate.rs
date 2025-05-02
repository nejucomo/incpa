/// A [ParserState](crate::state::ParserState) which can request recursive parsing
#[derive(Debug)]
pub enum RecursableState<S, C> {
    /// The inner [ParserState](crate::state::ParserState)
    InnerState(S),
    /// A request for recursive parsing with [Continuation](crate::recursive::Continuation) `C`
    ContinueWith(C),
}
