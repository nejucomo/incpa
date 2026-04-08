use crate::map::MapNext as _;
use crate::{ChompedResult, Input, Outcome, ParserState, UniversalParserError};

/// A type alias for a heap-allocated, type-erased [ParserState]
///
/// This is the concrete state type produced by recursive parsers, and is
/// the mechanism by which the infinite type-level recursion in a recursive grammar is broken.
///
/// See [DynParserState] for details on the underlying trait.
pub type BoxedParserState<I, O, E> = Box<dyn DynParserState<I, O, E>>;

/// An object-safe version of [ParserState]
///
/// Unlike [ParserState], this trait is object-safe because it uses [Box] instead of `Self`
/// for the continuation state returned by [DynParserState::feed_dyn]. This enables
/// type-erased incremental parsing, which is required to break the infinite type recursion
/// that would otherwise arise when defining recursive parsers.
///
/// This trait is automatically implemented for any type that implements [ParserState] whose
/// concrete type satisfies `'static`. Consumer code typically interacts with
/// [BoxedParserState] rather than this trait directly.
pub trait DynParserState<I: ?Sized, O, E>
where
    I: Input,
    E: From<UniversalParserError>,
{
    /// Object-safe variant of [ParserState::feed]
    ///
    /// Consumes the boxed state and returns either a new boxed continuation state or the
    /// parsed output, together with the number of input units consumed.
    fn feed_dyn(
        self: Box<Self>,
        input: &I,
    ) -> ChompedResult<Outcome<BoxedParserState<I, O, E>, O>, E>;

    /// Object-safe variant of [ParserState::end_input]
    ///
    /// Consumes the boxed state and signals that no more input is available.
    fn end_input_dyn(self: Box<Self>, final_input: &I) -> Result<O, E>;
}

impl<S, I: ?Sized, O, E> DynParserState<I, O, E> for S
where
    S: ParserState<Input = I, Output = O, Error = E> + 'static,
    I: Input,
    E: From<UniversalParserError>,
{
    fn feed_dyn(
        self: Box<Self>,
        input: &I,
    ) -> ChompedResult<Outcome<BoxedParserState<I, O, E>, O>, E> {
        (*self)
            .feed(input)
            .map_next(|s| Box::new(s) as BoxedParserState<I, O, E>)
    }

    fn end_input_dyn(self: Box<Self>, final_input: &I) -> Result<O, E> {
        (*self).end_input(final_input)
    }
}

impl<I: ?Sized, O, E> ParserState for BoxedParserState<I, O, E>
where
    I: Input,
    E: From<UniversalParserError>,
{
    type Input = I;
    type Output = O;
    type Error = E;

    fn feed(self, input: &I) -> ChompedResult<Outcome<Self, O>, E> {
        self.feed_dyn(input)
    }

    fn end_input(self, final_input: &I) -> Result<O, E> {
        self.end_input_dyn(final_input)
    }
}
