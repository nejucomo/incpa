#[derive(Debug)]
pub enum RecursingControl<S, C> {
    State(S),
    Cont(C),
}
