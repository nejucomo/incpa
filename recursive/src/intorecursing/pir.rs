use std::marker::PhantomData;

use derive_new::new;
use incpa::{Input, Parser, ParserCompose, ParserOutErr};

use crate::RecursingParser;
use crate::intorecursing::fstate::ParserStateAsFState;

#[derive(Debug, new)]
#[new(visibility = "pub(super)")]
pub struct ParserIntoRecursing<P, I, O>
where
    I: ?Sized + Input,
    P: Parser<I>,
{
    p: P,
    #[new(default)]
    ph: PhantomData<(Box<I>, O)>,
}

impl<P, I, O> RecursingParser<I, O> for ParserIntoRecursing<P, I, O>
where
    I: ?Sized + Input,
    P: Parser<I>,
{
    type FState = ParserStateAsFState<P::State, I, O>;

    fn start_recursing_parser(self) -> Self::FState {
        ParserStateAsFState::new(self.p.start_parser())
    }
}

impl<P, I, O> ParserCompose for ParserIntoRecursing<P, I, O>
where
    I: ?Sized + Input,
    P: Parser<I>,
{
}

impl<P, I, O> ParserOutErr for ParserIntoRecursing<P, I, O>
where
    I: ?Sized + Input,
    P: Parser<I>,
{
    type Output = P::Output;
    type Error = P::Error;
}
