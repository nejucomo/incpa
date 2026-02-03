use std::marker::PhantomData;

use derive_new::new;
use incpa_state::{Chomped, ChompedResult, Input, Outcome, ParserState, UniversalParserError};

use crate::Literal;

/// Parse a literal value
#[derive(Copy, Clone, Debug, new)]
pub struct LiteralState<I, L>
where
    I: ?Sized + Input,
    L: Literal<I>,
{
    literal: L,
    #[new(default)]
    ph: PhantomData<Box<I>>,
}

impl<I, L> ParserState for LiteralState<I, L>
where
    I: ?Sized + Input,
    L: Literal<I>,
{
    type Input = I;
    type Output = L;
    type Error = L::Error;

    fn feed(self, input: &Self::Input) -> ChompedResult<Outcome<Self, L>, Self::Error> {
        use UniversalParserError::UnexpectedInput;
        use incpa_state::Outcome::{Next, Parsed};

        let n = self.literal.literal_len();
        let prefix = input.prefix_up_to(n);

        if prefix.len() < n {
            Ok(Chomped::new(0, Next(self)))
        } else if self.literal.literal_eq(prefix) {
            Ok(Chomped::new(n, Parsed(self.literal)))
        } else {
            Err(Self::Error::from(UnexpectedInput))
        }
    }
}
