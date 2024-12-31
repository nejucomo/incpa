# incpa

`incpa` is an <u>inc</u>remental <u>pa</u>rser composition crate.

Incremental parsers process a chunk of input, then either produce an error, a parsed output, or an updated parser state ready for future input. This primitive, codified by [Parser::feed], allows the same parser definition to support parsing streaming input from async or sync sources, as well as other "incremental" use cases such as interactive REPL loop parsing.

The term "parser composition" emphasizes how sophisticated parsers can be defined by composing simpler parsers.

... [ ] TODO: Add example.

There is a fundamental trade-off between streaming parsers, such as this crate specializes in, versus "zero-copy" parsers which parse values which refer back to the original input buffer. Zero-copy parsers reduce the memory footprint and amount of copying at the cost of requiring all input to be held in memory, whereas streaming parsers can parse very large inputs at the cost of internally copying input where necessary.

## Related Projects

This crate is inspired by [chumsky](https://docs.rs/chumsky) which is an excellent and mature parser composition crate. Another inspiration is [parsec](https://hackage.haskell.org/package/parsec) in haskell-land. 

## Status

This crate is in the version 0.0.x phase of early proof of concept with unstable APIs.

### 0.1.0 Feature Goals

- [ ] A basic suite of general composition abstractions such as `Parser::then` and `Parser::or` with backtracking support.
- [ ] Support for both string parsers and slice parsers (including byte slices)
- [ ] Efficient streaming string parsing from byte-oriented I/O sources using UTF8 decoding
- [ ] Common generic primitive parsers, such as end-of-input, constants, and literals
- [ ] Common primitive text parsers, such as number literal parsers, whitespace parsers, keyword parsing, etc... 
- [ ] Common primitive byte-oriented parsers, such as integer types with different endianness, common variable-length integer encodings such as VLQ and LEB128, UTF8 chars, fixed-sized arrays, etc...
- [ ] Basic support for non-byte slice parsing with an example token slice parser.
- [ ] Location tracking in errors.
- [ ] Recursive parsers.
- [ ] Basic self benchmarks for comparison across revisions (but not necessarily comparison to alternative parser crates).
