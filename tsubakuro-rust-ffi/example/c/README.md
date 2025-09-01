# tsubakuro-rust-ffi C example

## How to generate C header file

```bash
cd tsubakuro-rust-ffi
cbindgen --lang C --config example/c/cbindgen.toml --output example/c/tsubakuro-rust-ffi.h
```

- [cbindgen](https://github.com/mozilla/cbindgen) 0.27.0

## How to build

```bash
cd tsubakuro-rust-ffi
cargo build --release

cd example/c/
gcc example.c -L../../target/release/ -ltsubakuro_rust_ffi -o example.out
```

## How to execute

Preload data into the table.

```sql
create table customer (
    c_id   bigint primary key,
    c_name varchar(30),
    c_age  int
);
insert into customer values(1, 'Hello', 51);
insert into customer values(2, 'World', 138);
insert into customer values(3, 'Tsurugi', 1);
```

```bash
cd tsubakuro-rust-ffi/example/c/
export LD_LIBRARY_PATH=../../target/release/
export RUST_LOG=tsubakuro_rust_ffi=trace
./example.out tcp://localhost:12345 user password
```

- See [env_logger](https://crates.io/crates/env_logger) for `RUST_LOG`.