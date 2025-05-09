//! Support for recursive parsers via [recursive_parser]
mod recursion;

pub use self::recursion::Recursion;

use crate::Parser;

/// Return a parser for a recursive grammar, defined by `F`
///
/// The closure `F` defines a recursive grammar given a [Recursion], which is an opaque parser for the grammar's output type. `F` itself uses this define the grammar, delegating to the [Recursion] whenever necessary.
///
/// # Example
///
/// Here's how we parse a simple toy example of balanced `[]` brackets with a single `-` in the middle. The output value is the number of matched `[]` pairs:
///
/// ```
#[doc = include_str!("../tests/recursive_smoke_test.rs")]
/// ```
pub fn recursive_parser<I, F, P>(f: F) -> impl Parser<I, Output = P::Output, Error = P::Error>
where
    I: ?Sized,
    F: FnOnce(Recursion<P::Output, P::Error>) -> P,
    P: Parser<I>,
{
    f(Recursion::default())
}
