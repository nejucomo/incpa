//! # Design Goals
//!
//! - Incrementally parse recursive grammars specified by composition methods.
//! - Layered crate depending on [incpa].
//! - Safe rust with `dyn` dispatch.
//! - [incpa::Parser] composition leverages recursive parser composition; therefore recursive parser components are [incpa::Parser]s.
//! - [incpa::Parser] compositions work out-of-the-box as recursive parser components, therefore their state has to "lift" continuations as a foreign trait impl.
#![deny(unsafe_code)]

#[cfg(test)]
mod tests;

mod arp;
mod cstate;
mod fimpls;
mod fstate;
mod intorecursing;
mod recstate;
mod recursing;
mod recursion;
mod recursive;

pub use self::arp::AutoRecursingParser;
pub use self::cstate::ContinueState;
pub use self::fstate::{AutoFeedState, FeedState};
pub use self::intorecursing::IntoRecursingParser;
pub use self::recstate::{ChompedRecOutcome, RecOutcome, RecState};
pub use self::recursing::RecursingParser;
pub use self::recursion::Recursion;
pub use self::recursive::recursive;
