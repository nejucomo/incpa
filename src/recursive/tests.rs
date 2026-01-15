use test_case::test_case;

use crate::recursive::{ContCtl, Continuation, RecursingState, RecursiveParserState};
use crate::state::{Chomped, FeedChomped, Outcome, ParserState};
use crate::{Parser, UniversalParserError};

#[test_case("[]" => Ok(SExpr::default()))]
fn test_parse(input: &str) -> Result<SExpr, UniversalParserError> {
    SExprParser.parse_all(input)
}

/// A minimal/toy type of "unit" S-expressions which demonstrates recursive grammar, e.g. `[[],[],[[],[]]]`
#[derive(Default, PartialEq)]
struct SExpr {
    children: Vec<SExpr>,
}

impl std::fmt::Debug for SExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.children.iter()).finish()
    }
}

struct SExprParser;

impl Parser<str> for SExprParser {
    type Output = SExpr;
    type Error = UniversalParserError;
    type State = RecursiveParserState<str, SExprRecursingState, SExpr>;

    fn start_parser(self) -> Self::State {
        RecursiveParserState::new(SExprRecursingState::default())
    }
}

/// A very finicky/minimal/strict parser for `SExpr`
#[derive(Debug, Default)]
struct SExprRecursingState {
    expecting: Expecting,
    sexpr: SExpr,
}

#[derive(Copy, Clone, Debug, Default)]
enum Expecting {
    #[default]
    Open,
    CloseOrComma,
}

impl RecursingState<str> for SExprRecursingState {
    type Continuation = SExpr;
    type CVal = SExpr;
    type RSOut = SExpr;
}

impl ParserState<str> for SExprRecursingState {
    type Output = ContCtl<Self, SExpr, SExpr>;
    type Error = UniversalParserError;

    fn feed(self, s: &str) -> Result<FeedChomped<Self, Self::Output>, Self::Error> {
        use Outcome::{Next, Parsed};

        let mut st = self;
        let mut chomptot = 0;

        for ch in s.chars() {
            let Chomped { consumed, value } = st.feed(&ch)?;
            chomptot += consumed;
            match value {
                Next(newst) => st = newst,
                Parsed(out) => return Ok(Chomped::new(chomptot, Parsed(out))),
            }
        }

        Ok(Chomped::new(s.len(), Next(st)))
    }

    fn end_input(self, final_input: &str) -> Result<Self::Output, Self::Error> {
        let _ = dbg!(final_input);
        Err(UniversalParserError::ExpectedMoreInput)
    }
}

impl ParserState<char> for SExprRecursingState {
    type Output = ContCtl<Self, SExpr, SExpr>;
    type Error = UniversalParserError;

    fn feed(self, c: &char) -> Result<FeedChomped<Self, Self::Output>, Self::Error> {
        use crate::state::Outcome::Parsed;
        use ContCtl::{Cont, Parsed as CParsed};
        use Expecting::{CloseOrComma, Open};
        use UniversalParserError::UnexpectedInput;

        let c = *c;
        dbg!(self.expecting, c);

        let res = match (&self.expecting, c) {
            (Open, '[') | (CloseOrComma, ',') => Ok(Parsed(Cont(self, SExpr::default()))),
            (CloseOrComma, ']') => Ok(Parsed(CParsed(self.sexpr))),
            _ => Err(UnexpectedInput),
        }
        .map(|oc| Chomped::new(c.len_utf8(), oc));

        dbg!(&res);
        res
    }
}

impl Continuation<str> for SExpr {
    type CState = SExprRecursingState;

    fn continue_with(
        mut self,
        v: SExpr,
    ) -> Result<Outcome<SExprRecursingState, SExpr>, UniversalParserError> {
        use crate::state::Outcome::Next;
        use Expecting::CloseOrComma;

        self.children.push(v);
        dbg!(Ok(Next(SExprRecursingState {
            expecting: CloseOrComma,
            sexpr: self,
        })))
    }
}
