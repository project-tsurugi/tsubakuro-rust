# tsubakuro-rust-core

tsubakuro-rust-core is the core library to access Tsurugi written in Rust.

## How to build

First, copy the proto files from [tsubakuro-proto](https://github.com/project-tsurugi/tsubakuro/tree/master/modules/proto).

```bash
cd tsubakuro-rust-core
cp -rp tsubakuro/modules/proto/src/main/protos .
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