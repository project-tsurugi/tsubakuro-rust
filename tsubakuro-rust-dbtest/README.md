# tsubakuro-rust-dbtest

tsubakuro-rust-dbtest tests [tsubakuro-rust-core](../tsubakuro-rust-core).
tsubakuro-rust-dbtest actually connects to tsurugidb.

## How to execute the example

```bash
cd tsubakuro-rust-dbtest
cargo run tcp://localhost:12345 user password
```

- arg1
  - endpoint url
- arg2
  - user for credential
- arg3
  - password for user

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
- user=`<user>`
  - user for credential
- password=`<password>`
  - password for user
- auth-token=`<token>`
- credentials=`</path/to/credential-file>`