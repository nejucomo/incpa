//! The `Map*` traits

/// Types which enable mapping a next state  `T` value
pub trait MapNext<T>: Sized {
    /// The result of mapping the parsed value
    type MappedNext<U>;

    /// Map the parsed value with `f`
    fn map_next<F, U>(self, f: F) -> Self::MappedNext<U>
    where
        F: FnOnce(T) -> U;
}

/// Types which enable mapping a parsed `T` value
pub trait MapParsed<T>: Sized {
    /// The type which applied or will apply the mapping function, `F`
    type MappedParsed<U>;

    /// Map the parsed value
    fn map_parsed<F, U>(self, f: F) -> Self::MappedParsed<U>
    where
        F: FnOnce(T) -> U;
}

/// Types which enable adjusting a consumed `usize`
pub trait MapConsumed: Sized {
    /// Map the consumed amount
    fn map_consumed<F>(self, f: F) -> Self
    where
        F: FnOnce(usize) -> usize;
}

/// Types which enable mapping an outcome `T`
pub trait MapOutcome<T>: Sized {
    /// The mapped type
    type MappedOutcome<U>;

    /// Map the Outcome amount
    fn map_outcome<F, U>(self, f: F) -> Self::MappedOutcome<U>
    where
        F: FnOnce(T) -> U;
}
