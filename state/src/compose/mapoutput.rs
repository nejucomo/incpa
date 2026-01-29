use incpa_compose::MapOutput;

use crate::map::MapOutcome as _;
use crate::{ChompedResult, Outcome, ParserState};

impl<P, F, O> ParserState for MapOutput<P, F, O>
where
    P: ParserState,
    F: FnOnce(P::Output) -> O,
{
    fn feed(self, input: &Self::Input) -> ChompedResult<Outcome<Self, O>, Self::Error> {
        use crate::Outcome::{Next, Parsed};

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
