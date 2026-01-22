use derive_new::new;
use incpa::state::{Chomped, Outcome};

pub type ChompedRecOutcome<F, C, O> = Chomped<RecOutcome<F, C, O>>;

pub type RecOutcome<F, C, O> = Outcome<RecState<F, C>, O>;

#[derive(Debug, new)]
pub struct RecState<S, C> {
    pub fstate: S,
    pub optcont: Option<C>,
}
