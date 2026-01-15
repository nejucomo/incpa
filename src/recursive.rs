//! # Notes
//!
//! - The continuation needs to be inserted into the output if we rely on [ParserState] because we can't do `S -> Result<FeedChomped<???, ...>, ...>` where `???` is either `S` or a continuation.
//! -
#![allow(missing_docs, clippy::type_complexity)]

#[cfg(test)]
mod tests;

use std::marker::PhantomData;

use derive_new::new;

use crate::state::{Chomped, FeedChomped, Outcome, ParserState};

// === traits and types === \\

#[derive(Debug, new)]
#[new(visibility = "")]
pub struct RecursiveParserState<I, S, O>
where
    I: ?Sized,
    S: RecursingState<I, RSOut = O, CVal = O>,
{
    recursing: S,
    #[new(default)]
    pending: Vec<S::Continuation>,
    #[new(default)]
    ph: PhantomData<I>,
}

pub trait RecursingState<I>:
    std::fmt::Debug + ParserState<I, Output = ContCtl<Self, Self::Continuation, Self::RSOut>>
where
    I: ?Sized,
{
    type Continuation: Continuation<I, CState = Self>;
    type CVal;
    type RSOut;
}

#[derive(Debug)]
pub enum ContCtl<RS, C, P> {
    Cont(RS, C),
    Parsed(P),
}

pub trait Continuation<I>: Sized + std::fmt::Debug
where
    I: ?Sized,
{
    type CState: RecursingState<I, Continuation = Self>;

    fn continue_with(
        self,
        v: <Self::CState as RecursingState<I>>::CVal,
    ) -> Result<ContinuationOutcome<Self::CState, I>, <Self::CState as ParserState<I>>::Error>;
}

pub type ContinuationOutcome<RS, I> = Outcome<RS, <RS as RecursingState<I>>::RSOut>;

// === impls === \\
impl<I, S, O> ParserState<I> for RecursiveParserState<I, S, O>
where
    I: ?Sized,
    S: RecursingState<I, RSOut = O, CVal = O, Error: std::fmt::Debug>,
{
    type Output = S::RSOut;
    type Error = S::Error;

    fn feed(self, input: &I) -> Result<FeedChomped<Self, Self::Output>, Self::Error> {
        let Self {
            mut recursing,
            mut pending,
            ..
        } = self;

        let mut chomptot = 0;

        loop {
            use Outcome::{Next, Parsed};

            let Chomped {
                consumed,
                value: outcome,
            } = recursing.feed(input)?;

            chomptot += consumed;

            match outcome {
                Next(next) => {
                    recursing = next;
                }
                Parsed(ContCtl::Parsed(out)) => {
                    return pop_continuations(pending, out)
                        .map(|outcome| Chomped::new(chomptot, outcome));
                }
                Parsed(ContCtl::Cont(next, c)) => {
                    pending.push(c);
                    recursing = next;
                }
            }
        }
    }
}

fn pop_continuations<I, S, O>(
    mut pending: Vec<S::Continuation>,
    out0: O,
) -> Result<Outcome<RecursiveParserState<I, S, O>, O>, S::Error>
where
    I: ?Sized,
    S: RecursingState<I, RSOut = O, CVal = O, Error: std::fmt::Debug>,
{
    use Outcome::{Next, Parsed};

    let mut out = out0;
    while let Some(cont) = pending.pop() {
        match cont.continue_with(out)? {
            Next(recursing) => {
                return Ok(Next(RecursiveParserState {
                    recursing,
                    pending,
                    ph: PhantomData,
                }));
            }
            Parsed(outn) => {
                out = outn;
            }
        }
    }
    Ok(Parsed(out))
}
