# Merriam Webster HTTP
An HTTP client for the Merriam Webster's Dictionary API

[![CI](https://github.com/Fyko/merriam-webster/actions/workflows/ci.yml/badge.svg)](https://github.com/Fyko/merriam-webster/actions/workflows/ci.yml)
[![Crate](https://img.shields.io/crates/v/merriam-webster-http.svg)](https://crates.io/crates/merriam-webster-http)
[![API](https://docs.rs/merriam-webster-http/badge.svg)](https://docs.rs/merriam-webster-http)

# Example
```rust,no_run
use std::env;

use merriam_webster_http::MerriamWebsterClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let api_key = env::var("MERRIAM_WEBSTER_API_KEY")?;
    let client = MerriamWebsterClient::new(api_key.into());

    let defs = client
        .collegiate_definition("documentation".to_string())
        .await?;
    let def = defs.first()
        .expect(format!("No definition found for {}", word).as_str());
    let shortdefs = def.shortdef.as_ref().unwrap();

    println!("Short definitions for 'documentation': {shortdefs:#?}");

    Ok(())
}
```

## Features

### TLS

**Note**: not enabling any TLS feature is supported for use behind a proxy;
Merrium Webster's API is HTTPS only.

**Note**: this TLS code was taken from [twilight-http] in accodance with its license.

`merriam-webster-http` has features to enable HTTPS connectivity with [`hyper`]. These
features are mutually exclusive. `rustls-native-roots` is enabled by default.

#### `native`

The `native` feature uses a HTTPS connector provided by [`hyper-tls`].

To enable `native`, do something like this in your `Cargo.toml`:

```toml
[dependencies]
merriam-webster-http = { default-features = false, features = ["native"], version = "0.1" }
```

#### `rustls-native-roots`

The `rustls-native-roots` feature uses a HTTPS connector provided by [`hyper-rustls`], which uses
[`rustls`] as the TLS backend, and enables its `native-tokio` feature, which uses [`rustls-native-certs`]
for root certificates.

This is enabled by default.

#### `rustls-webpki-roots`

The `rustls-webpki-roots` feature uses a HTTPS connector provided by [`hyper-rustls`], which uses
[`rustls`] as the TLS backend, and enables its `webpki-tokio` feature, which uses [`webpki-roots`]
for root certificates.

This should be preferred over `rustls-native-roots` in Docker containers based on `scratch`.

### Trust-DNS

The `trust-dns` enables [`hyper-trust-dns`], which replaces the default
`GaiResolver` in [`hyper`]. [`hyper-trust-dns`] instead provides a fully
async DNS resolver on the application level.

[`hyper`]: https://crates.io/crates/hyper
[`hyper-rustls`]: https://crates.io/crates/hyper-rustls
[`hyper-tls`]: https://crates.io/crates/hyper-tls
[`rustls`]: https://crates.io/crates/rustls
[`rustls-native-certs`]: https://crates.io/crates/rustls-native-certs
[`hyper-trust-dns`]: https://crates.io/crates/hyper-trust-dns
[`webpki-roots`]: https://crates.io/crates/webpki-roots
[twilight-http]: https://github.com/twilight-rs/twilight/tree/main/twilight-http

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
