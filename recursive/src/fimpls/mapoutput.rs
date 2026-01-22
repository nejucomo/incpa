use std::marker::PhantomData;
use std::rc::Rc;

use derive_new::new;
use incpa::combinators::MapOutput;
use incpa::state::{ChompedExt as _, OutcomeExt as _};
use incpa::{Input, ParserCompose, ParserOutErr};

use crate::{ChompedRecOutcome, ContinueState, FeedState, RecOutcome, RecState, RecursingParser};

impl<I, P, O, F, R> RecursingParser<I, R> for MapOutput<P, F, O>
where
    I: ?Sized + Input,
    P: RecursingParser<I, R>,
    F: FnOnce(P::Output) -> O,
{
    type FState = RecMapOutput<P::FState, P::Output, F, O>;

    fn start_recursing_parser(self) -> Self::FState {
        RecMapOutput::new(self.inner.start_recursing_parser(), Rc::new(self.f))
    }
}

#[derive(Debug, new)]
#[new(visibility = "")]
pub struct RecMapOutput<P, O, F, FO> {
    /// The inner parser
    inner: P,
    /// The mapping function
    f: Rc<F>,
    #[new(default)]
    ph: PhantomData<(O, FO)>,
}

impl<P, O, F, FO> Clone for RecMapOutput<P, O, F, FO>
where
    P: Clone,
{
    fn clone(&self) -> Self {
        RecMapOutput::new(self.inner.clone(), self.f.clone())
    }
}

impl<I, S, O, F, FO, R> FeedState<I, R> for RecMapOutput<S, O, F, FO>
where
    I: ?Sized + Input,
    S: FeedState<I, R, Output = O>,
    F: FnOnce(O) -> FO,
{
    type CState = RecMapOutput<S::CState, O, F, FO>;

    fn feed_recursing(
        self,
        input: &I,
    ) -> Result<ChompedRecOutcome<Self, Self::CState, Self::Output>, Self::Error> {
        self.inner
            .feed_recursing(input)
            .map_value(recout_mapper(self.f))
    }
}

impl<I, C, O, F, FO, R> ContinueState<I, R> for RecMapOutput<C, O, F, FO>
where
    I: ?Sized + Input,
    C: ContinueState<I, R, Output = O>,
    F: FnOnce(O) -> FO,
{
    type FState = RecMapOutput<C::FState, O, F, FO>;

    fn continue_with(
        self,
        cval: R,
    ) -> Result<RecOutcome<Self::FState, Self, Self::Output>, Self::Error> {
        self.inner.continue_with(cval).map(recout_mapper(self.f))
    }
}

#[allow(clippy::type_complexity)]
fn recout_mapper<S, C, O, F, FO>(
    f: Rc<F>,
) -> impl FnOnce(
    RecOutcome<S, C, O>,
) -> RecOutcome<RecMapOutput<S, O, F, FO>, RecMapOutput<C, O, F, FO>, FO>
where
    F: FnOnce(O) -> FO,
{
    use incpa::state::Outcome::{Next, Parsed};

    const RC_INVARIANT_ERROR: &str =
        "RecMapOutput invariant failure: all `F` refs dropped except final prior to call.";

    |recout| match recout {
        Next(RecState { fstate, optcont }) => Next(RecState::new(
            RecMapOutput::new(fstate, f.clone()),
            optcont.map(|c| RecMapOutput::new(c, f)),
        )),
        Parsed(out) => {
            let bare_f = Rc::into_inner(f).expect(RC_INVARIANT_ERROR);
            Parsed(bare_f(out))
        }
    }
}

impl<S, O, F, FO> ParserCompose for RecMapOutput<S, O, F, FO>
where
    S: ParserCompose,
    F: FnOnce(O) -> FO,
{
}

impl<S, O, F, FO> ParserOutErr for RecMapOutput<S, O, F, FO>
where
    S: ParserOutErr,
    F: FnOnce(O) -> FO,
{
    type Output = FO;
    type Error = S::Error;
}
