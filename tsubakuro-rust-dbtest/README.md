# tsubakuro-rust-dbtest

tsubakuro-rust-dbtest tests [tsubakuro-rust-client](../tsubakuro-rust-client).
tsubakuro-rust-dbtest actually connects to tsurugidb.

## How to execute

```bash
cd tsubakuro-rust-dbtest
cargo run tcp://localhost:12345
```

- arg1
  - endpoint url

## How to test

```bash
cd tsubakuro-rust-dbtest
cargo test "" -- --test-threads=1 endpoint=tcp://localhost:12345
```

- arg1
  - test function name
- --test-threads=1
  - tsubakuro-rust-dbtest test functions cannot be executed in parallel.
- endpoint=`<url>`
  - endpoint url