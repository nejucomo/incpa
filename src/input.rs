/// A common interface many parsers use for manipulating input
pub trait Input {
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

    /// Return the longest prefix with length `<= n`
    fn prefix_up_to(&self, n: usize) -> &Self;
}

impl Input for str {
    fn len(&self) -> usize {
        self.len()
    }

    fn split_at(&self, n: usize) -> (&Self, &Self) {
        str::split_at(self, n)
    }

    fn prefix_up_to(&self, n: usize) -> &Self {
        let n = std::cmp::min(n, self.len());
        let (prefix, _) = self.split_at(std::cmp::min(self.len(), n));
        prefix
    }
}

impl<T> Input for [T] {
    fn len(&self) -> usize {
        <[T]>::len(self)
    }

    fn split_at(&self, n: usize) -> (&Self, &Self) {
        <[T]>::split_at(self, n)
    }

    fn prefix_up_to(&self, n: usize) -> &Self {
        let n = std::cmp::min(n, self.len());
        &self[..n]
    }
}
