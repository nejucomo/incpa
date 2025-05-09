#[test]
fn smoke_test() {
    let parser = '['
        .then(Recursion::default())
        .then(']')
        .map(|((_, depth), _)| depth + 1)
        .or('|'.map(|_| 0));

    let actual = parser.parse_all("[[[|]]]").unwrap();
    let expected = 3;

    assert_eq!(actual, expected);
}
