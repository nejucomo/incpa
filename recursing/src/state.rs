use incpa_ioe::{IncpaIOE, UniversalParserError::ExpectedMoreInput};
use incpa_state::{ChompedResult, ParserState};

use crate::{Continuation, RecursingOutcome, RecursiveState};

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

pub trait AutoRecursingState<O>: Clone + RecursingState<O, Output = O> {
    fn into_parser_state(self) -> impl ParserState<Output = O> {
        RecursiveState::new(self)
    }
}

impl<B, O> AutoRecursingState<O> for B where B: Clone + RecursingState<O, Output = O> {}
