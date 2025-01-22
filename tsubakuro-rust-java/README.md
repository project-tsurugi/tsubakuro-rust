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
./gradlew test
```

