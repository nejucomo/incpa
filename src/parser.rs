use crate::Update;

pub trait Parser<I, O, E>: Sized
where
    I: ?Sized,
{
    /// Feed an input reference to the parser to produce an update
    fn feed(self, input: &I) -> Result<Update<Self, O>, E>;

    /// Unwrap a pending output
    fn unwrap_pending(self) -> Option<O>;
}
