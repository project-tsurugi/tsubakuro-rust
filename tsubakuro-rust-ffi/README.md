# tsubakuro-rust-ffi

tsubakuro-rust-ffi is a library providing functions in C ABI format. (Functions called from other programming languages)

Depends on [tsubakuro-rust-core](../tsubakuro-rust-core).

## How to build

```bash
cd tsubakuro-rust-ffi
cargo build --release
ls target/release/
```

## How to generate C header file

```bash
cd tsubakuro-rust-ffi
cbindgen --lang C --output tsubakuro-rust-ffi.h
```

- [cbindgen](https://github.com/mozilla/cbindgen)

There is an example in [tsubakuro-rust-java](../tsubakuro-rust-java).