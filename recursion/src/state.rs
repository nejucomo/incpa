use std::marker::PhantomData;

use derive_new::new;

use incpa::state::Outcome::{self, Next, Parsed};
use incpa::state::{ChompedExt as _, FeedChomped, ParserState};

use crate::Step::{ParsedRec, RequestRec};
use crate::{Continuation, RecursiveParser};

/// A [RecursionPivot] is a [ParserState] which internally dispatches to [`P::Continuation`] when necessary to produce a final parsed value
#[derive(Debug, new)]
#[new(visibility = "")]
pub struct RecursionPivot<P, I, O>
where
    P: RecursiveParser<I, O, O>,
    I: ?Sized,
{
    parser: P,
    state: P::State,
    pending: Vec<P::Continuation>,
    #[new(default)]
    ph: PhantomData<O>,
}

impl<P, I, O> From<P> for RecursionPivot<P, I, O>
where
    P: RecursiveParser<I, O, O>,
    I: ?Sized,
{
    fn from(parser: P) -> Self {
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

    fn feed(self, input: &I) -> Result<FeedChomped<Self, Self::Output>, Self::Error> {
        let RecursionPivot {
            parser,
            state,
            mut pending,
            ph: _,
        } = self;

        let mk_next = |parser, state, pending| -> Result<Outcome<Self, O>, Self::Error> {
            Ok(Next(RecursionPivot::new(parser, state, pending)))
        };

        state
            .feed(input)
            .map_value(|outcome: Outcome<_, _>| -> Result<_, _> {
                match outcome {
                    Next(state) => mk_next(parser, state, pending),

                    Parsed(mut step) => loop {
                        match step {
                            ParsedRec(output) => {
                                if let Some(c) = pending.pop() {
                                    match c.recurse_from(output)? {
                                        Next(state) => {
                                            return mk_next(parser, state, pending);
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
                                return mk_next(parser, state, pending);
                            }
                        }
                    },
                }
            })
            .and_then(|chomped| chomped.transpose())
    }
}
