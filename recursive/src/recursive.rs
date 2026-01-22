mod parser;
mod state;

use incpa::{Input, Parser};

use crate::{AutoRecursingParser, Recursion};

pub use self::parser::RecursiveParser;
pub use self::state::RecursiveState;

pub fn recursive<O, F, P, I>(mk_parser: F) -> impl Parser<I, Output = O>
where
    F: FnOnce(Recursion<P::Output>) -> P,
    P: AutoRecursingParser<I, Output = O>,
    I: ?Sized + Input,
{
    RecursiveParser::new(mk_parser(Recursion::default()))
}
