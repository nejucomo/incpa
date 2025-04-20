# incpa

`incpa` is an <u>inc</u>remental <u>pa</u>rser composition crate.

Incremental parsers process a chunk of input, then either produce an error, a parsed output, or an updated parser state ready for future input. This primitive, codified by [ParserState::feed](crate::state::ParserState::feed), allows the same parser definition to support parsing streaming input from async or sync sources, as well as other "incremental" use cases such as interactive REPL loop parsing.

The term "parser composition" emphasizes how sophisticated parsers can be defined by composing simpler parsers.

## Related Crates

The `incpa` project functionality is separated into multiple distinct crates:

- [`incpa-byte`](https://docs.rs/incpa-byte): byte-oriented parsing, parsers, and input
- [`incpa-str`](https://docs.rs/incpa-str): str-oriented parsing, parsers, and input
- [`incpa-tokio`](https://docs.rs/incpa-tokio): support for async streaming input via [`tokio`](https://docs.rs/tokio)`::io::AsyncRead` sources

## Example

```rust
use incpa::BaseParserError;
use incpa::primitive::remaining;
use incpa::Parser;

fn main() -> Result<(), BaseParserError> {
    let parser = define_my_parser();
    let output = parser.parse_all("Hello World!")?;
    assert_eq!(output, ("Hello", " World!".to_string()));
    Ok(())
}

fn define_my_parser() -> impl Parser<str, Output=(&'static str, String), Error=BaseParserError> {
    "Hello".then(remaining())
}
```

## Trade-offs

There is a fundamental trade-off between streaming parsers, like `incpa`-based parsers, versus "zero-copy" parsers which parse values which refer back to the original input buffer.

Zero-copy parsers reduce the memory footprint and amount of copying at the cost of requiring all input to be held in memory, whereas streaming parsers can parse very large inputs at the cost of internally copying input where necessary.

## Related Projects

This crate is inspired by [chumsky](https://docs.rs/chumsky) which is an excellent and mature parser composition crate. Another inspiration is [parsec](https://hackage.haskell.org/package/parsec) in haskell-land. 

## Status

This crate is in the version 0.0.x phase of early proof of concept with unstable APIs.

### Roadmap

<details>
<summary>Roadmap</summary>

#### 0.1.0 Feature Goals

- [x] A basic suite of general composition abstractions such as [Parser::then] and [Parser::or] with backtracking support. (As of v0.0.1.)
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

- [Parser], [Parser::map], [Parser::map_error], [Parser::then], [Parser::or]
- [state::ParserState]
- `incpa_byte::ByteParser`, hardcoded `incpa_byte::BufferManager` strategy (later moved to separate `incpa_byte` crate.

</details>
