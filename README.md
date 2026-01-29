# incpa

`incpa` is an <u>inc</u>remental <u>pa</u>rser composition crate.

Incremental parsers process a chunk of input, then either produce an error, a parsed output, or an updated parser state ready for future input. This primitive, codified by [ParserState::feed](incpa_state::ParserState::feed), allows the same parser definition to support parsing streaming input from async or sync sources, as well as other "incremental" use cases such as interactive REPL loop parsing.

The term "parser composition" emphasizes how sophisticated parsers can be defined by composing simpler parsers.

## Example

```rust
use incpa_compose::ParserCompose;
use incpa_parser::Parser;
use incpa_parser::primitive::remaining;
use incpa_ioe::UniversalParserError;

fn main() -> Result<(), UniversalParserError> {
    let parser = define_my_parser();
    let output = parser.parse_all("Hello World!")?;
    assert_eq!(output, ("Hello", " World!".to_string()));
    Ok(())
}

fn define_my_parser() -> impl Parser<Input=str, Output=(&'static str, String), Error=UniversalParserError> {
    "Hello".then(remaining())
}
```

## Related Crates

This [incpa](crate) provides an "all-batteries included" API by re-exporting multiple `incpa-` prefixed crates into one namespace. Projects which prefer having fewer direct dependencies or new projects might start by depending on just this top-level crate, while projects which want to exclude code and build-times can select the subset of re-exported crates they rely on directly. 

Each of the mods in this crate re-export a crate with an exact naming correspondence, e.g. the [parser] mod → the [incpa_parser] crate, the [state] mod → the [incpa_state] crate, and so on.

## Related Projects

This crate is inspired by [chumsky](https://docs.rs/chumsky) which is an excellent and mature parser composition crate. Another inspiration is [parsec](https://hackage.haskell.org/package/parsec) in haskell-land. 

## Status

This crate is in the version 0.0.x phase of early proof of concept with unstable APIs.

### Roadmap

<details>
<summary>Roadmap</summary>

#### 0.1.0 Feature Goals

- [x] A basic suite of general composition abstractions such as `Parser::then` and `Parser::or` with backtracking support. (As of v0.0.1.)
- [x] Support for both string parsers and slice parsers (including byte slices). (As of v0.0.1.)
- [x] Efficient streaming string parsing from byte-oriented I/O sources using UTF8 decoding. (As of v0.0.1.)
- [ ] Tunable buffer management.
- [ ] Common generic primitive parsers, such as end-of-input, constants, and literals.
- [ ] Common primitive text parsers, such as number literal parsers, whitespace parsers, keyword parsing, etc...
- [ ] Common primitive byte-oriented parsers, such as integer types with different endianness, common variable-length integer encodings such as VLQ and LEB128, UTF8 chars, fixed-sized arrays, etc...
- [ ] Basic support for non-byte slice parsing with an example token slice parser.
- [ ] Location tracking in errors.
- [ ] Recursive parsers.
- [ ] Thorough test coverage.
- [ ] Proof-of-API plausibility with a major parser application or two in downstream crates.
- [ ] Basic self benchmarks for comparison across revisions (but not necessarily comparison to alternative parser crates).

</details>

### Changelog

<details>
<summary>Changelog</summary>

#### v0.0.3 (Not Yet Released)

- Split out `incpa-byte` crate.

#### v0.0.2

This release just fixed some missing `Cargo.toml` metadata: `homepage` and `repository`.

#### v0.0.1

Basic core structure with:

- `Parser`, `Parser::map`, `Parser::map_error`, `Parser::then`, `Parser::or`
- `state::ParserState`
- `incpa_byte::ByteParser`, hardcoded `incpa_byte::BufferManager` strategy (later moved to separate `incpa_byte` crate.

</details>
