use incpa_state::Outcome;

#[derive(Debug)]
pub enum RecursingControl<S, C> {
    State(S),
    Cont(C),
}

pub type RecursingOutcome<S, C, O> = Outcome<RecursingControl<S, C>, O>;
