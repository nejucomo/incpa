#![doc = include_str!("../Description.md")]
#![doc = include_str!("../../README-subcrate-link.md")]
#![deny(missing_docs, unsafe_code)]
// TODO: Remove this:
#![allow(missing_docs)]

mod continuation;
mod control;
mod recursive;
mod state;

pub use self::continuation::Continuation;
pub use self::control::{RecursingControl, RecursingOutcome};
pub use self::recursive::RecursiveState;
pub use self::state::{AutoRecursingState, RecursingState};
