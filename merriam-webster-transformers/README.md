# Merriam Webster Transformers
Parsers and transformers for non-standard text markup in Merriam-Webster's Collegiate Dictionary API

[![CI](https://github.com/Fyko/merriam-webster/actions/workflows/ci.yml/badge.svg)](https://github.com/Fyko/merriam-webster/actions/workflows/ci.yml)
[![Crate](https://img.shields.io/crates/v/merriam-webster-transformers.svg)](https://crates.io/crates/merriam-webster-transformers)
[![Docs](https://docs.rs/merriam-webster-transformers/badge.svg)](https://docs.rs/merriam-webster-transformers)

## Example
> Coming Soon
```rust,ignore
use merriam_webster_transformers::{parse_string, transformers::Markdown};

let input = "{it}monitors{\/it} and instructors for troops green in the art of war";
let parsed = parse_string(input.to_string())?;
let transformed = Markdown::transform(parsed)?;

println!("{transformed}");
// *monitors* and instructors for troops green in the art of war
```

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
