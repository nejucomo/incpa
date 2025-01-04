//! Incrementally parse inputs into intermediate states, outputs, or errors
mod buffer;
mod bufmgr;
mod outcome;
mod parser;
mod resultimpls;
mod update;

pub use self::buffer::Buffer;
pub use self::bufmgr::BufferManager;
pub use self::outcome::{Outcome, OutcomeExt};
pub use self::parser::Parser;
pub use self::update::{Update, UpdateExt};
