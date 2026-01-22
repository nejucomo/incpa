use std::convert::Infallible;
use std::marker::PhantomData;

use incpa::state::ParserState;
use incpa::{Input, ParserOutErr};

use crate::intorecursing::fstate::ParserStateAsFState;
use crate::{ContinueState, RecOutcome};

#[derive(Debug)]
pub struct NeverCState<S, I, O>
where
    I: ?Sized + Input,
    S: ParserState<I>,
{
    #[allow(dead_code)]
    uninstantiatable: Infallible,
    ph: PhantomData<(S, Box<I>, O)>,
}

impl<S, I, O> ParserOutErr for NeverCState<S, I, O>
where
    I: ?Sized + Input,
    S: ParserState<I>,
{
    type Output = S::Output;
    type Error = S::Error;
}

impl<S, I, O> ContinueState<I, O> for NeverCState<S, I, O>
where
    I: ?Sized + Input,
    S: ParserState<I>,
{
    type FState = ParserStateAsFState<S, I, O>;

    fn continue_with(
        self,
        _: O,
    ) -> Result<RecOutcome<Self::FState, Self, Self::Output>, Self::Error> {
        unreachable!()
    }
}
