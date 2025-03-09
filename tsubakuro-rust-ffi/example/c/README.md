# tsubakuro-rust-ffi C example

## How to generate C header file

```bash
cd tsubakuro-rust-ffi
cbindgen --lang C --config example/c/cbindgen.toml --output example/c/tsubakuro-rust-ffi.h
```

## How to build

```bash
cd tsubakuro-rust-ffi
cargo build

cd example/c/
gcc example.c -L../../target/debug/ -ltsubakuro_rust_ffi -o example.out
```

## How to execute

```bash
cd tsubakuro-rust-ffi/example/c/
export LD_LIBRARY_PATH=../../target/debug/
export RUST_LOG=tsubakuro_rust_ffi=trace
./example.out tcp://localhost:12345
```

