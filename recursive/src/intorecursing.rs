mod cstate;
mod fstate;
mod pir;

use incpa::{Input, Parser};

use crate::AutoRecursingParser;

pub trait IntoRecursingParser<I: ?Sized + Input>: Parser<I> {
    fn into_recursing_parser(self) -> impl AutoRecursingParser<I> {
        self::pir::ParserIntoRecursing::new(self)
    }
}

impl<P, I> IntoRecursingParser<I> for P
where
    P: Parser<I>,
    I: ?Sized + Input,
{
}
