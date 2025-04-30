use crate::parse_recursive;

#[test]
fn recursive_parser_smoke_test() {
    parse_recursive(|r| {
        '['.then(r)
            .then(']')
            .map(|((_, n), _)| n + 1)
            .or('|'.map(|_| 0))
    })
}
