# tsubakuro-rust-core

tsubakuro-rust-core is the core library to access [Tsurugi](https://github.com/project-tsurugi/tsurugidb) for Rust.

tsubakuro-rust-core is a port from [Tsubakuro/Java](https://github.com/project-tsurugi/tsubakuro), but it does not cover all functions.

## Target

- Tsurugi 1.5.0 or later.

## Limitations

- Provide SQL service only.
- Only TCP connection is available.

## Crate features

Default feature include the following features.

- `with_bigdecimal` - Enable decimal via [bigdecimal](https://crates.io/crates/bigdecimal).
- `with_rust_decimal` - Enable decimal via [rust_decimal](https://crates.io/crates/rust_decimal).
- `with_chrono` - Enable date/time via [chrono](https://crates.io/crates/chrono).
- `with_time` - Enable date/time via [time](https://crates.io/crates/time).

## Rust version requirements

The Minimum Supported Rust Version (MSRV) is currently **Rust 1.84.1**.

## How to use

Add `tsubakuro-rust-core` as a dependency to your `Cargo.toml` file:

```toml
[dependencies]
tsubakuro-rust-core = "0.4.0"
```



## Example

### connect example

```rust
use std::time::Duration;
use log::warn;
use tsubakuro_rust_core::prelude::*;

async fn example() -> Result<(), TgError> {
    let endpoint = Endpoint::parse("tcp://localhost:12345")?;

    let mut connection_option = ConnectionOption::new();
    connection_option.set_endpoint(endpoint);
    connection_option.set_application_name("Tsubakuro/Rust example");
    connection_option.set_session_label("example session");
    connection_option.set_default_timeout(Duration::from_secs(10));

    // connect
    let session = Session::connect(&connection_option).await?;

    // make SqlClient
    let client: SqlClient = session.make_client();

    // execute SQL
    let result = example_transaction(&client).await;

    // session close
    if let Err(e) = session.close().await {
        warn!("session close error. {}", e);
    }

    result
}
```

See [example.rs](https://github.com/project-tsurugi/tsubakuro-rust/blob/master/tsubakuro-rust-dbtest/src/bin/example2.rs) for more examples.



## How to build

Need [protoc](https://github.com/protocolbuffers/protobuf?tab=readme-ov-file#protobuf-compiler-installation) command since used [prost](https://crates.io/crates/prost).  
(For example, to install `protoc` on Ubuntu 22.04, execute `apt install protobuf-compiler`)

If proto files in [tsubakuro-proto](https://github.com/project-tsurugi/tsubakuro/tree/master/modules/proto) has been modified, copy from there.

```bash
cd tsubakuro-rust-core
cp -rp /path/to/tsubakuro/modules/proto/src/main/protos .
```

Then build with `cargo`.

```bash
cd tsubakuro-rust-core
cargo build
```

## How to test

```bash
cd tsubakuro-rust-core
cargo test
```

See also [tsubakuro-rust-dbtest](../tsubakuro-rust-dbtest).

## License

[Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)