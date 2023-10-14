# Merriam Webster
Rust libraries for the [Merriam Webster Collegiate Dictionary API].

[![CI](https://github.com/Fyko/merriam-webster/actions/workflows/ci.yml/badge.svg)](https://github.com/Fyko/merriam-webster/actions/workflows/ci.yml)

## Packages
- [`merriam-webster-http`]: An HTTP client for the Merriam Webster's Dictionary API
- [`merriam-webster-model`]: Models for the Merriam Webster's Dictionary API


[Merriam Webster Collegiate Dictionary API]: https://dictionaryapi.com/products/json
[`merriam-webster-http`]: ./merriam-webster-http
[`merriam-webster-model`]: ./merriam-webster-model


## Examples

### Definition
Fetches the short definitions for the word "documentation".
```console
$ MERRIAM_WEBSTER_API_KEY=... cargo run --example definition documentation
```

### Top Words
Queries the current top words for Merrium Webster's searchbar.
```console
$ MERRIAM_WEBSTER_API_KEY=... cargo run --example top_words
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
