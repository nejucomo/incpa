use std::marker::PhantomData;

use derive_new::new;
use incpa::{Input, Parser, ParserCompose, ParserOutErr};

use crate::AutoRecursingParser;
use crate::recursive::RecursiveState;

#[derive(Debug, new)]
#[new(visibility = "pub(crate)")]
pub struct RecursiveParser<P, I>
where
    I: ?Sized + Input,
    P: AutoRecursingParser<I>,
{
    arp: P,
    #[new(default)]
    ph: PhantomData<Box<I>>,
}

impl<P, I> ParserOutErr for RecursiveParser<P, I>
where
    I: ?Sized + Input,
    P: AutoRecursingParser<I>,
{
    type Output = P::Output;
    type Error = P::Error;
}

impl<P, I> ParserCompose for RecursiveParser<P, I>
where
    I: ?Sized + Input,
    P: AutoRecursingParser<I>,
{
}

impl<P, I> Parser<I> for RecursiveParser<P, I>
where
    I: ?Sized + Input,
    P: AutoRecursingParser<I>,
{
    type State = RecursiveState<P::FState, I>;

    fn start_parser(self) -> Self::State {
        RecursiveState::new(self.arp.start_recursing_parser())
    }
}
