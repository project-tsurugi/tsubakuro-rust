# tsubakuro-rust-java

tsubakuro-rust-java is a project for testing [tsubakuro-rust-ffi](../tsubakuro-rust-ffi).

## How to build

First, generate a C header file from [tsubakuro-rust-ffi](../tsubakuro-rust-ffi).

```bash
cd tsubakuro-rust-ffi
cbindgen --lang C --config ../tsubakuro-rust-java/src/main/c/cbindgen.toml --output ../tsubakuro-rust-java/src/main/c/tsubakuro-rust-ffi.h
```

- [cbindgen](https://github.com/mozilla/cbindgen)

Then generate Java source from C header file.

```bash
cd tsubakuro-rust-java
jextract -t com.tsurugidb.tsubakuro.rust.ffi --output src/generated/java src/main/c/tsubakuro-rust-ffi.h
```

- [jextract](https://github.com/openjdk/jextract)

## How to test

```bash
cd tsubakuro-rust-java
./gradlew test -Pffi.library.path=/path/to/libtsubakuro_rust_ffi.so -Pdbtest.endpoint=tcp://localhost:12345 -Pdbtest.endpoint.java=ipc:tsurugi
```

or

```bash
export TSURUGI_FFI_LIBRARY_PATH=/path/to/libtsubakuro_rust_ffi.so
cd tsubakuro-rust-java
./gradlew test -Pdbtest.endpoint=tcp://localhost:12345 -Pdbtest.endpoint.java=ipc:tsurugi
```

- `libtsubakuro_rust_ffi.so` (`tsubakuro_rust_ffi.dll` for MS-Windows) is a file built with [tsubakuro-rust-ffi](../tsubakuro-rust-ffi).
- If `-Pdbtest.endpoint` is omitted, it will be `tcp://localhost:12345`.
- If `-Pdbtest.endpoint.java` is omitted, it is same as value of `-Pdbtest.endpoint`.

### test with credential

```bash
./gradlew test -Pffi.library.path=/path/to/libtsubakuro_rust_ffi.so \
-Pdbtest.endpoint=tcp://localhost:12345 \
-Pdbtest.user=user \
-Pdbtest.password=password \
-Pdbtest.auth-token=token \
-Pdbtest.credentials=/path/to/credential-file
```

For tests other than credential, specifying only one of `user`, `auth-token`, or `credentials` is sufficient. If none of these are specified, authentication will be performed using the user `tsurugi`.

In the credential test, anything not specified is skipped.

### test with lob path mapping

Lob path mapping for privileged mode is specified using the format `<client-path>:<server-path>`.

#### example for MS-Windows command prompt

```
docker container run -d -p 12345:12345 -p 52345:52345 -v C:/tmp/client:/mnt/client -v C:/tmp/tsurugi:/opt/tsurugi/var/data/log --name tsurugi -e GLOG_v=30 ghcr.io/project-tsurugi/tsurugidb:latest

gradlew test -Pffi.library.path=/path/to/libtsubakuro_rust_ffi.dll ^
-Pdbtest.lob-send-path-mapping=C:/tmp/client:/mnt/client ^
-Pdbtest.lob-recv-path-mapping=C:/tmp/tsurugi:/opt/tsurugi/var/data/log
```

### test with blob relay service endpoint

#### example

```bash
./gradlew test -Pffi.library.path=/path/to/libtsubakuro_rust_ffi.so \
-Pdbtest.blob-relay-service-endpoint=http:///localhost:52345
```

