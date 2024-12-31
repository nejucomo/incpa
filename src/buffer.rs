/// A common interface many parsers use for manipulating input
pub trait Buffer {
    /// Return a number of "items" contained in the referenced input
    fn len(&self) -> usize;

    /// Whether or not there are any items in the buffer
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Return the suffix of this buffer after dropping `n` items
    ///
    /// # Panics
    ///
    /// If `n` is larger than the buffer size, this panics.
    fn drop_prefix(&self, n: usize) -> &Self;
}

impl<T> Buffer for [T] {
    fn len(&self) -> usize {
        <[T]>::len(self)
    }

    fn drop_prefix(&self, n: usize) -> &Self {
        &self[n..]
    }
}
