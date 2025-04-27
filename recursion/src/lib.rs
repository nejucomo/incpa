//! Recursive [incpa] parser support via recursion continuations
#![deny(missing_docs, unsafe_code)]
mod cont;
mod parser;
mod state;
mod step;

pub use self::cont::Continuation;
pub use self::parser::ParseRecursive;
pub use self::state::ParseRecursiveState;
pub use self::step::Step;
