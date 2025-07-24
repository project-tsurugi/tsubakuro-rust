# tsubakuro-rust-odbc (Tsurugi ODBC Driver)

tsubakuro-rust-odbc is a ODBC Driver to access [Tsurugi](https://github.com/project-tsurugi/tsurugidb).

## Limitation

- Tsurugi 1.5.0 or later.
- ODBC3
- For MS-Windows, 64-bit DLL
- transaction_type is OCC only.
- commit_option is default only.
- shutdown_type is always GRACEFUL.
- BLOB/CLOB is not supported.

## How to build

```bash
cd tsubakuro-rust-odbc
cargo build --release
dir target\release\*tsubakuro_rust_odbc*
```

`tsubakuro_rust_odbc.dll` for MS-Windows is generated.

## License

[Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)

