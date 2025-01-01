/// A common interface many parsers use for manipulating input
pub trait Buffer {
    /// Produce an empty buffer
    fn empty() -> &'static Self
    where
        Self: 'static;

    /// Return a number of "items" contained in the referenced input
    fn len(&self) -> usize;

    /// Whether or not there are any items in the buffer
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Split at the given offset
    fn split_at(&self, n: usize) -> (&Self, &Self);

    /// Return the suffix of this buffer after dropping `n` items
    ///
    /// # Panics
    ///
    /// If `n` is larger than the buffer size, this panics.
    fn drop_prefix(&self, n: usize) -> &Self {
        self.split_at(n).1
    }
}

impl<T> Buffer for [T] {
    fn empty() -> &'static Self
    where
        Self: 'static,
    {
        &[]
    }

    fn len(&self) -> usize {
        <[T]>::len(self)
    }

    fn split_at(&self, n: usize) -> (&Self, &Self) {
        <[T]>::split_at(self, n)
    }
}

impl Buffer for str {
    fn empty() -> &'static Self
    where
        Self: 'static,
    {
        ""
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn split_at(&self, n: usize) -> (&Self, &Self) {
        str::split_at(self, n)
    }
}
