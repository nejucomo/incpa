use std::marker::PhantomData;

use derive_new::new;
use incpa::state::{ChompedExt as _, FeedChomped, ParserState};
use incpa::{Input, ParserOutErr};

use crate::{AutoFeedState, ContinueState};

#[derive(Debug, new)]
#[new(visibility = "pub(super)")]
pub struct RecursiveState<S, I>
where
    S: AutoFeedState<I>,
    I: ?Sized + Input,
{
    fstate: S,
    #[new(default)]
    cs: Vec<S::CState>,
    #[new(default)]
    ph: PhantomData<(Box<I>, S)>,
}

impl<S, I> ParserOutErr for RecursiveState<S, I>
where
    S: AutoFeedState<I>,
    I: ?Sized + Input,
{
    type Output = S::Output;
    type Error = S::Error;
}

impl<S, I> ParserState<I> for RecursiveState<S, I>
where
    S: AutoFeedState<I>,
    I: ?Sized + Input,
{
    fn feed(self, input: &I) -> Result<FeedChomped<Self, Self::Output>, Self::Error> {
        use crate::RecState;
        use incpa::state::Outcome::*;

        let Self { fstate, mut cs, ph } = self;

        let fch = fstate
            .feed_recursing(input)
            .map_value(|recstate| -> Result<_, S::Error> {
                match recstate {
                    Next(RecState { fstate, optcont }) => {
                        if let Some(c) = optcont {
                            cs.push(c);
                        }
                        Ok(Next(Self { fstate, cs, ph }))
                    }
                    Parsed(mut val) => {
                        while let Some(c) = cs.pop() {
                            match c.continue_with(val)? {
                                Next(RecState { fstate, optcont }) => {
                                    if let Some(c) = optcont {
                                        cs.push(c);
                                    }
                                    return Ok(Next(Self { fstate, cs, ph }));
                                }
                                Parsed(innerval) => {
                                    val = innerval;
                                }
                            }
                        }
                        Ok(Parsed(val))
                    }
                }
            })
            // TODO: nightly `Result::flatten` after `Chomped::transpose`
            .map(|ch| ch.transpose())??;

        Ok(fch)
    }
}
