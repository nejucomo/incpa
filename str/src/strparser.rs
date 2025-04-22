use incpa::Parser;

impl<P> StrParser for P where P: Parser<str> {}

/// A parser of `&str` input
pub trait StrParser: Parser<str> {}
