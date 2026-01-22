use std::marker::PhantomData;

use derive_new::new;
use incpa::state::{OutcomeExt as _, ParserState};
use incpa::{Input, ParserOutErr};

use crate::intorecursing::cstate::NeverCState;
use crate::{ChompedRecOutcome, FeedState, RecState};

#[derive(Debug, new)]
#[new(visibility = "pub(super)")]
pub struct ParserStateAsFState<S, I, O>
where
    I: ?Sized + Input,
    S: ParserState<I>,
{
    s: S,
    #[new(default)]
    ph: PhantomData<(Box<I>, O)>,
}

impl<S, I, O> ParserOutErr for ParserStateAsFState<S, I, O>
where
    I: ?Sized + Input,
    S: ParserState<I>,
{
    type Output = S::Output;
    type Error = S::Error;
}

impl<S, I, O> FeedState<I, O> for ParserStateAsFState<S, I, O>
where
    I: ?Sized + Input,
    S: ParserState<I>,
{
    type CState = NeverCState<S, I, O>;

    fn feed_recursing(
        self,
        input: &I,
    ) -> Result<ChompedRecOutcome<Self, Self::CState, Self::Output>, Self::Error> {
        self.s.feed(input).map_parser(|inner| RecState {
            fstate: ParserStateAsFState::new(inner),
            // There is never a continuation:
            optcont: None,
        })
    }
}
