//! # Design Goals
//!
//! - Incrementally parse recursive grammars specified by composition methods.
//! - Layered crate depending on [incpa].
//! - Safe rust with `dyn` dispatch.
//! - [incpa::Parser] composition leverages recursive parser composition; therefore recursive parser components are [incpa::Parser]s.
//! - [incpa::Parser] compositions work out-of-the-box as recursive parser components, therefore their state has to "lift" continuations as a foreign trait impl.
#![deny(unsafe_code)]

use incpa::state::{Chomped, Outcome};
use incpa::{Input, ParserCompose, UniversalParserError};

#[cfg(test)]
mod tests;

pub trait RecursiveParser<I: ?Sized + Input>: ParserCompose {
    type FState: FeedState<I, Self::Output, Output = Self::Output, Error = Self::Error>;
}

pub trait FeedState<I: ?Sized + Input, C>: Sized {
    /// The type of output on successful parse
    type Output;

    /// The type of errors this parser detects
    type Error: From<UniversalParserError>;

    type CState: ContinueState<I, C, FState = Self, Output = Self::Output, Error = Self::Error>;

    /// Feed an input reference to the parser to produce an update
    ///
    /// Precondition: `input` includes a suffix which has not been seen previously by this parser.
    #[allow(clippy::type_complexity)]
    fn feed(
        self,
        input: &I,
    ) -> Result<Chomped<RecOutcome<Self, Self::CState, Self::Output>>, Self::Error>;
}

pub type RecOutcome<F, C, O> = Outcome<RecState<F, C>, O>;

pub enum RecState<S, C> {
    State(S),
    Cont(C),
}

pub trait ContinueState<I: ?Sized + Input, C>: Sized {
    /// The type of output on successful parse
    type Output;

    /// The type of errors this parser detects
    type Error: From<UniversalParserError>;

    type FState: FeedState<I, C>;

    fn continue_with(
        self,
        cval: C,
    ) -> Result<RecOutcome<Self::FState, Self, Self::Output>, Self::Error>;
}
