use crate::Step;

pub trait Continuation<R, O>: Sized {
    fn recurse_from(self, parsed: R) -> Step<O, Self>;
}
