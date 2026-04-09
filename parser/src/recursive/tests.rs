use incpa_state::UniversalParserError;

use crate::compose::ParserCompose as _;
use crate::recursive::recursive;
use crate::Parser;

// Grammar: balanced = "ab" | "a" balanced "b"
// Matches strings of the form a^n b^n (n >= 1).
fn balanced_parser(
) -> impl Parser<str, Output = &'static str, Error = UniversalParserError> + Clone {
    recursive::<str, &'static str, UniversalParserError, _, _>(|rec| {
        "ab".or("a".then(rec).then("b").map_output(|((_, inner), _)| inner))
    })
}

#[test]
fn base_case() {
    assert_eq!(balanced_parser().parse_all("ab").unwrap(), "ab");
}

#[test]
fn one_level_deep() {
    assert_eq!(balanced_parser().parse_all("aabb").unwrap(), "ab");
}

#[test]
fn two_levels_deep() {
    assert_eq!(balanced_parser().parse_all("aaabbb").unwrap(), "ab");
}

#[test]
fn mismatch_returns_error() {
    assert!(balanced_parser().parse_all("ba").is_err());
}

#[test]
fn non_recursive_define_works() {
    // A `recursive` call whose closure ignores the handle is just a regular parser.
    let p = recursive::<str, &'static str, UniversalParserError, _, _>(|_rec| "hello");
    assert_eq!(p.parse_all("hello world").unwrap(), "hello");
}

#[test]
fn weak_clone_works_while_owner_alive() {
    let p = balanced_parser();
    // Clone produces a Weak-backed handle; it must work while `p` is alive.
    let cloned = p.clone();
    assert_eq!(cloned.parse_all("aabb").unwrap(), "ab");
    // `p` is still alive here, so the clone above was valid.
    drop(p);
}
