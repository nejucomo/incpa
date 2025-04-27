//! Recursive [incpa] parser support via recursion continuations
#![deny(missing_docs, unsafe_code)]
mod cont;
mod parser;
mod state;
mod step;

pub use self::cont::Continuation;
pub use self::parser::{RecursiveParser, parse_recursive};
pub use self::state::RecursionPivot;
pub use self::step::Step;
