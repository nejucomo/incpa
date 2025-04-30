//! Recursive [incpa] parser support via recursion continuations
#![deny(missing_docs, unsafe_code)]
mod cont;
mod parser;
mod prec;
mod recursion;
mod state;
mod step;

pub use self::cont::Continuation;
pub use self::parser::RecursiveParser;
pub use self::prec::{ParseRecursive, parse_recursive};
pub use self::recursion::Recursion;
pub use self::state::RecursionPivot;
pub use self::step::Step;

#[cfg(test)]
mod tests;
