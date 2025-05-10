use incpa::Parser as _;
use incpa_recursive::recursive_parser;

fn main() {
    let p = recursive_parser(|r| {
        '['.then(r)
            .then(']')
            .map(|((_, depth), _)| depth + 1)
            .or('-'.map(|_| 0))
            // Both cases return a usize, so just return a usize:
            .map(|either| either.into_inner())
    });

    let depth = p.parse_all("[[[-]]]").unwrap();
    assert_eq!(depth, 3);
}
