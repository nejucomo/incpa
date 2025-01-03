use std::fmt::Debug;

use test_case::test_case;

use crate::parsing::{Buffer, Parser};
use crate::syntax::Literal;

#[test_case("Hello", "Hello World!")]
#[test_case(b"Hello", b"Hello World!".as_slice())]
fn literal_ok<L, I>(literal: L, input: &I) -> anyhow::Result<()>
where
    L: Literal<I, anyhow::Error> + PartialEq + Debug,
    I: ?Sized + Buffer,
{
    let output = literal.into_parser().parse(input)?;
    assert_eq!(literal, output);
    Ok(())
}
