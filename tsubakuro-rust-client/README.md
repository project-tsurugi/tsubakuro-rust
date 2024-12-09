# tsubakuro-rust-client

tsubakuro-rust-client is the main library to access Tsurugi written in Rust.

## How to build

First, copy the proto files from [tsubakuro-proto](https://github.com/project-tsurugi/tsubakuro/tree/master/modules/proto).

```bash
cd tsubakuro-rust-client
cp -rp tsubakuro/modules/proto/src/main/protos .
```

Then build with `cargo`.

```bash
cd tsubakuro-rust-client
cargo build
```

