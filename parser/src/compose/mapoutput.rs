use std::marker::PhantomData;

use derive_new::new;
use incpa_state::map::MapOutcome as _;
use incpa_state::{ChompedResult, Input, Outcome, ParserState};

use crate::Parser;

/// Specifies a parser which maps its output
#[derive(Copy, Clone, Debug, new)]
pub struct MapOutput<P, F, O> {
    /// The inner parser or state
    pub inner: P,
    /// The output mapping function
    pub f: F,
    #[new(default)]
    ph: PhantomData<O>,
}

impl<I, P, F, O> Parser<I> for MapOutput<P, F, O>
where
    I: ?Sized + Input,
    P: Parser<I>,
    F: FnOnce(P::Output) -> O,
{
    type State = MapOutput<P::State, F, O>;
    type Output = O;
    type Error = P::Error;

    fn start_parser(self) -> Self::State {
        todo!()
    }
}

impl<S, F, O> ParserState for MapOutput<S, F, O>
where
    S: ParserState,
    F: FnOnce(S::Output) -> O,
{
    type Input = S::Input;
    type Output = O;
    type Error = S::Error;

    fn feed(self, input: &Self::Input) -> ChompedResult<Outcome<Self, O>, Self::Error> {
        use Outcome::{Next, Parsed};

        let MapOutput { inner, f, .. } = self;

        inner.feed(input).map_outcome(|oc| match oc {
            Next(p) => Next(MapOutput::new(p, f)),
            Parsed(o) => Parsed(f(o)),
        })
    }

    fn end_input(self, final_input: &Self::Input) -> Result<O, Self::Error> {
        self.inner.end_input(final_input).map(self.f)
    }
}
