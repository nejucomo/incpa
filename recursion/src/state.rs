use std::marker::PhantomData;

use derive_new::new;
use incpa::Parser;
use incpa::state::Outcome::{Next, Parsed};
use incpa::state::{ParserState, Update, UpdateExt as _};

use crate::Continuation;
use crate::Step::{self, ParsedRec, RequestRec};

#[derive(Debug, new)]
pub struct ParseRecursiveState<P, S, C, O> {
    parser: P,
    inner: S,
    #[new(default)]
    pending: Vec<C>,
    #[new(default)]
    ph: PhantomData<O>,
}

impl<I, P, S, C, O> ParserState<I> for ParseRecursiveState<P, S, C, O>
where
    P: Clone + Parser<I, State = S>,
    S: ParserState<I, Output = Step<O, C>>,
    C: Continuation<O, O>,
{
    type Output = O;
    type Error = S::Error;

    fn feed(self, input: &I) -> Result<Update<Self, Self::Output>, Self::Error> {
        let ParseRecursiveState {
            parser,
            inner,
            mut pending,
            ph,
        } = self;

        inner.feed(input).map_outcome(|oc| match oc {
            Next(inner) => Next(ParseRecursiveState {
                parser,
                inner,
                pending,
                ph,
            }),
            Parsed(mut step) => loop {
                match step {
                    ParsedRec(output) => {
                        if let Some(c) = pending.pop() {
                            step = c.recurse_from(output);
                        } else {
                            return Parsed(output);
                        }
                    }
                    RequestRec(c) => {
                        pending.push(c);
                        let inner = parser.clone().into_parser();
                        return Next(ParseRecursiveState {
                            parser,
                            inner,
                            pending,
                            ph,
                        });
                    }
                }
            },
        })
    }
}
