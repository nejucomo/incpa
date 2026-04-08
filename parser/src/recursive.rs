#[cfg(test)]
mod tests;

use std::cell::OnceCell;
use std::rc::{Rc, Weak};

use incpa_state::map::MapNext as _;
use incpa_state::{
    BoxedParserState, ChompedResult, Input, Outcome, ParserState,
    UniversalParserError,
};

use crate::Parser;

/// Internal type alias for the factory cell held by [Recursive]
type FactoryCell<I, O, E> = OnceCell<Box<dyn Fn() -> BoxedParserState<I, O, E>>>;

/// Internal storage for [Recursive]: either the owning strong reference or a weak clone
enum RecursiveInner<I: ?Sized, O, E> {
    /// The original, uniquely-owned reference produced by [recursive]
    Owned(Rc<FactoryCell<I, O, E>>),
    /// A borrowed weak reference produced by [Clone] or given to the `define` closure
    Borrowed(Weak<FactoryCell<I, O, E>>),
}

/// A parser that can reference itself recursively in its own definition
///
/// Construct one with the [recursive] free function.
///
/// # Cloning
///
/// Cloning a `Recursive` produces a weak-backed handle. Weak clones remain usable as long
/// as the original `Recursive` value (or a [ParserState] derived from it) is still alive,
/// but they do not extend the lifetime of the underlying factory. This design avoids the
/// reference-count cycle that would otherwise arise when the parser definition captures a
/// clone of itself.
pub struct Recursive<I: ?Sized, O, E> {
    inner: RecursiveInner<I, O, E>,
}

impl<I: ?Sized, O, E> Clone for Recursive<I, O, E> {
    /// Produces a weak-backed clone; see the [type-level docs](Recursive) for details.
    fn clone(&self) -> Self {
        let weak = match &self.inner {
            RecursiveInner::Owned(rc) => Rc::downgrade(rc),
            RecursiveInner::Borrowed(weak) => Weak::clone(weak),
        };
        Recursive {
            inner: RecursiveInner::Borrowed(weak),
        }
    }
}

impl<I, O, E> Parser<I> for Recursive<I, O, E>
where
    I: ?Sized + Input + 'static,
    O: 'static,
    E: From<UniversalParserError> + 'static,
{
    type State = BoxedParserState<I, O, E>;
    type Output = O;
    type Error = E;

    fn start_parser(self) -> BoxedParserState<I, O, E> {
        let cell = match self.inner {
            RecursiveInner::Owned(rc) => rc,
            RecursiveInner::Borrowed(weak) => weak
                .upgrade()
                .expect("Recursive parser used after its definition was dropped"),
        };
        // Return a lazily-initialised state.  The factory is NOT called here; it is
        // called on the first `feed` or `end_input` call.  This is what prevents
        // infinite recursion during parser-state construction.
        Box::new(RecursiveState { cell, inner: None })
    }
}

/// The in-progress parsing state of a [Recursive] parser
///
/// Lazily initialises the real inner state from the factory on the first call to
/// [ParserState::feed] or [ParserState::end_input]. This deferred initialisation is
/// what prevents infinite type-level and value-level recursion during construction.
pub(crate) struct RecursiveState<I: ?Sized, O, E> {
    cell: Rc<FactoryCell<I, O, E>>,
    inner: Option<BoxedParserState<I, O, E>>,
}

impl<I: ?Sized, O, E> RecursiveState<I, O, E> {
    /// Initialise the inner state if it has not been initialised yet
    fn init_inner(inner: Option<BoxedParserState<I, O, E>>, cell: &Rc<FactoryCell<I, O, E>>) -> BoxedParserState<I, O, E> {
        inner.unwrap_or_else(|| {
            let factory = cell
                .get()
                .expect("Recursive parser used before its definition was completed");
            factory()
        })
    }
}

impl<I: ?Sized, O, E> ParserState for RecursiveState<I, O, E>
where
    I: Input + 'static,
    O: 'static,
    E: From<UniversalParserError> + 'static,
{
    type Input = I;
    type Output = O;
    type Error = E;

    fn feed(self, input: &I) -> ChompedResult<Outcome<Self, O>, E> {
        let Self { cell, inner } = self;
        let state = Self::init_inner(inner, &cell);
        state
            .feed_dyn(input)
            .map_next(|next| RecursiveState {
                cell,
                inner: Some(next),
            })
    }

    fn end_input(self, final_input: &I) -> Result<O, E> {
        let Self { cell, inner } = self;
        let state = Self::init_inner(inner, &cell);
        state.end_input(final_input)
    }
}

/// Define a recursive parser
///
/// The `define` closure receives a weak-backed handle to the parser being defined.  The
/// handle can be stored anywhere in the grammar definition (e.g. passed to `.then()` or
/// `.or()`), but it must not be used to start parsing until after `recursive` returns.
///
/// # Panics
///
/// - If `start_parser` is called on the handle *inside* the `define` closure (the
///   factory cell has not been set yet).
/// - If a cloned handle is used after the owning `Recursive` value and all states derived
///   from it have been dropped.
///
/// # Memory
///
/// Cloning `Recursive` produces a weak reference, so the parser definition does not form a
/// strong reference cycle.  The factory cell is kept alive as long as the owning
/// `Recursive` value or any [BoxedParserState] derived from it is alive.
///
/// # Example
///
/// ```rust
/// use incpa_parser::{Parser, ParserCompose as _};
/// use incpa_parser::recursive::recursive;
/// use incpa_state::UniversalParserError;
///
/// // Grammar: balanced = "ab" | "a" balanced "b"
/// // Matches: "ab", "aabb", "aaabbb", …
/// let parser = recursive::<str, &'static str, UniversalParserError, _, _>(|rec| {
///     "ab".or(
///         "a".then(rec)
///             .then("b")
///             .map_output(|((_, inner), _)| inner),
///     )
/// });
///
/// assert_eq!(parser.parse_all("aabb").unwrap(), "ab");
/// ```
pub fn recursive<I, O, E, P, F>(define: F) -> Recursive<I, O, E>
where
    I: ?Sized + Input + 'static,
    P: Parser<I, Output = O, Error = E> + Clone + 'static,
    P::State: 'static,
    O: 'static,
    E: From<UniversalParserError> + 'static,
    F: FnOnce(Recursive<I, O, E>) -> P,
{
    let cell: Rc<FactoryCell<I, O, E>> = Rc::new(OnceCell::new());

    // Give the closure a weak-backed handle so that the closure captured inside the
    // factory does not form a strong Rc cycle with the cell.
    let handle = Recursive {
        inner: RecursiveInner::Borrowed(Rc::downgrade(&cell)),
    };

    let concrete = define(handle);

    cell.set(Box::new(move || {
        let state = concrete.clone().start_parser();
        Box::new(state) as BoxedParserState<I, O, E>
    }))
    .ok()
    .expect("recursive cell was already initialised");

    Recursive {
        inner: RecursiveInner::Owned(cell),
    }
}
