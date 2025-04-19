use std::fmt::Debug;

use test_case::test_case;

use crate::parsing::Outcome::{Next, Parsed};
use crate::parsing::{Buffer, ParserState, Update};

use super::Literal;

#[test_case("Hello", "Hello World!")]
#[test_case(b"Hello", b"Hello World!".as_slice())]
fn literal_ok<L, I>(literal: L, input: &I) -> anyhow::Result<()>
where
    L: Literal<I> + PartialEq + Debug,
    L::Error: std::error::Error + Send + Sync + 'static,
    L::State: Debug,
    I: ?Sized + Buffer,
{
    let Update { consumed, outcome } = literal.into_parser().feed(input)?;
    assert_eq!(consumed, literal.literal_len());
    match outcome {
        Next(p) => panic!("unexpected {p:?}"),
        Parsed(output) => {
            assert_eq!(literal, output);
            Ok(())
        }
    }
}
