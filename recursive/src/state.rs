use derive_new::new;
use incpa::state::{FeedChomped, ParserState};

/// Handle recursion for `S`
#[derive(Debug, new)]
pub struct RecursiveParserState<S> {
    inner: S,
    #[new(default)]
    continuations: Vec<()>,
}

impl<I, S> ParserState<I> for RecursiveParserState<S>
where
    I: ?Sized,
    S: ParserState<I>,
{
    type Output = S::Output;
    type Error = S::Error;

    fn feed(self, input: &I) -> Result<FeedChomped<Self, Self::Output>, Self::Error> {
        let _ = (self.inner, self.continuations, input);
        todo!()
    }
}
