# Tsubakuro/Rust - Rust library for Tsurugi

Tsubakuro/Rust is a Rust library that accesses the Tsurugi database.

C ABI format library is also provided.

## libraries

- [tsubakuro-rust-client](tsubakuro-rust-client)
  - Main library to access Tsurugi.
  - Written in Rust.
- [tsubakuro-rust-ffi](tsubakuro-rust-ffi)
  - Library providing functions in C ABI format. (Functions called from other programming languages)
  - Written in Rust.
  - Using tsubakuro-rust-client.
- [tsubakuro-rust-java](tsubakuro-rust-java)
  - For testing tsubakuro-rust-ffi.
  - Written in Java.
  - Using tsubakuro-rust-ffi.