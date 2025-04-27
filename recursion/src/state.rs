use std::marker::PhantomData;

use incpa::state::Outcome::{Next, Parsed};
use incpa::state::{ParserState, Update, UpdateExt as _};

use crate::Step::{ParsedRec, RequestRec};
use crate::{Continuation, RecursiveParser};

/// A [RecursionPivot] is a [ParserState] which internally dispatches to [`P::Continuation`] when necessary to produce a final parsed value
#[derive(Debug)]
pub struct RecursionPivot<P, I, O>
where
    P: RecursiveParser<I, O, O>,
    I: ?Sized,
{
    parser: P,
    state: P::State,
    pending: Vec<P::Continuation>,
    ph: PhantomData<O>,
}

impl<P, I, O> RecursionPivot<P, I, O>
where
    P: RecursiveParser<I, O, O>,
    I: ?Sized,
{
    /// Construct a new state from a [RecursiveParser]
    pub fn new(parser: P) -> Self {
        RecursionPivot {
            parser: parser.clone(),
            state: parser.into_parser(),
            pending: vec![],
            ph: PhantomData,
        }
    }
}

impl<P, I, O> ParserState<I> for RecursionPivot<P, I, O>
where
    P: RecursiveParser<I, O, O>,
    I: ?Sized,
{
    type Output = O;
    type Error = P::Error;

    fn feed(self, input: &I) -> Result<Update<Self, Self::Output>, Self::Error> {
        let RecursionPivot {
            parser,
            state,
            mut pending,
            ph,
        } = self;

        state.feed(input).try_map_outcome(|oc| match oc {
            Next(state) => Ok(Next(RecursionPivot {
                parser,
                state,
                pending,
                ph,
            })),
            Parsed(mut step) => loop {
                match step {
                    ParsedRec(output) => {
                        if let Some(c) = pending.pop() {
                            match c.recurse_from(output)? {
                                Next(state) => {
                                    return Ok(Next(RecursionPivot {
                                        parser,
                                        state,
                                        pending,
                                        ph,
                                    }));
                                }
                                Parsed(next_step) => {
                                    step = next_step;
                                }
                            }
                        } else {
                            return Ok(Parsed(output));
                        }
                    }
                    RequestRec(c) => {
                        pending.push(c);
                        let state = parser.clone().into_parser();
                        return Ok(Next(RecursionPivot {
                            parser,
                            state,
                            pending,
                            ph,
                        }));
                    }
                }
            },
        })
    }
}
