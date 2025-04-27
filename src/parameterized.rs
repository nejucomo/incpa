use crate::Parser;

/// A [ParameterizedParser] defines a [Parser] given a parameter, `P`
pub trait ParameterizedParser<P, I> {
    /// The [Parser] defined with parameter `P`
    type Parser: Parser<I>;

    /// Define the [Parser] with parameter `P`
    fn parser_with(self, param: P) -> Self::Parser;
}
