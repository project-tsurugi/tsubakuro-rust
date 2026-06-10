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
- lob-send-path-mapping=`<client_path>:<server_path>`
- lob-recv-path-mapping=`<client_path>:<server_path>`

#### Example of LOB path mapping in MS-Windows command prompt

```
docker container run -d -p 12345:12345 -p 52345:52345 -v C:/tmp/client:/mnt/client -v C:/tmp/tsurugi:/opt/tsurugi/var/data/log --name tsurugi -e GLOG_v=30 ghcr.io/project-tsurugi/tsurugidb:latest

cargo test "" -- --test-threads=1 endpoint=tcp://localhost:12345 ^
lob-send-path-mapping=C:/tmp/client:/mnt/client ^
lob-recv-path-mapping=C:/tmp/tsurugi:/opt/tsurugi/var/data/log
```

#### Example of blob relay service endpoint

```bash
cargo test "" -- --test-threads=1 endpoint=tcp://localhost:12345 blob-relay-service-endpoint=http://localhost:52345
```

> [!IMPORTANT]
>
> Tsubakuro/Rust (the [tonic](https://docs.rs/tonic/latest/tonic/) crate used by Tsubakuro/Rust) does not support `dns:///`.
> Use `http://` instead.

#### Example of blob relay service CA certificate PEM file

```bash
cargo test "" -- --test-threads=1 endpoint=tcp://localhost:12345 blob-relay-service-endpoint=https://localhost:52345 blob-relay-service-ca-cert-pem=/path/to/pem
```
