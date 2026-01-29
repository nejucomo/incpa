use incpa_ioe::{IncpaIOE, UniversalParserError::ExpectedMoreInput};
use incpa_state::{ChompedResult, Outcome};

use crate::{Continuation, RecursingControl};

pub trait RecursingState<R>: IncpaIOE {
    type Continuation: Continuation<Self, R>;

    fn feed_recursingly(
        self,
        input: &Self::Input,
    ) -> ChompedResult<RecursingOutcome<Self, Self::Continuation, Self::Output>, Self::Error>;

    fn end_input_recursingly(self, final_input: &Self::Input) -> Result<Self::Output, Self::Error> {
        let _ = final_input;
        Err(Self::Error::from(ExpectedMoreInput))
    }
}

pub type RecursingOutcome<S, C, O> = Outcome<RecursingControl<S, C>, O>;
