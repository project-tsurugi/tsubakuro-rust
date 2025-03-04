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