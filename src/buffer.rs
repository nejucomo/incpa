/// A common interface many parsers use for manipulating input
pub trait Buffer {
    /// Return a number of "items" contained in the referenced input
    fn len(&self) -> usize;

    /// Whether or not there are any items in the buffer
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
