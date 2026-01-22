use incpa::{Input, Parser, ParserOutErr};

use crate::RecursingParser;
use crate::recursive::RecursiveParser;

pub trait AutoRecursingParser<I: ?Sized + Input>:
    RecursingParser<I, <Self as ParserOutErr>::Output>
{
    fn into_parser(self) -> impl Parser<I> {
        RecursiveParser::new(self)
    }
}

impl<B, I: ?Sized + Input> AutoRecursingParser<I> for B where
    B: RecursingParser<I, <Self as ParserOutErr>::Output>
{
}
