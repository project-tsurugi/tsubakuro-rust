# tsubakuro-rust-odbc-dbtest

tsubakuro-rust-odbc-dbtest is a project for testing [tsubakuro-rust-odbc](../tsubakuro-rust-odbc).

## How to test

`odbc32.dll` must be included in the PATH.

```bash
cd tsubakuro-rust-odbc-dbtest
./gradlew test -Pdbtest.odbc.lib.name=odbc32 -Pdbtest.endpoint=tcp://localhost:12345 -Pdbtest.endpoint.java=ipc:tsurugi
```

- If `-Pdbtest.odbc.lib.name` is omitted, it will be `odbc32`.
- If `-Pdbtest.endpoint` is omitted, it will be `tcp://localhost:12345`.
- If `-Pdbtest.endpoint.java` is omitted, it is same as value of `-Pdbtest.endpoint`.