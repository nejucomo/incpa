pub trait Continuation<S, R> {
    fn continue_with(recval: R) -> S;
}
