pub trait Continuation<S, R> {
    fn continue_with(self, recval: R) -> S;
}
