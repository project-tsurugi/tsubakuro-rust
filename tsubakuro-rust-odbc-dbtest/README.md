# tsubakuro-rust-odbc-dbtest

tsubakuro-rust-odbc-dbtest is a project for testing [tsubakuro-rust-odbc](../tsubakuro-rust-odbc).

## How to test

`odbc32.dll` must be included in the PATH.

```bash
cd tsubakuro-rust-odbc-dbtest
./gradlew test -Pdbtest.odbc.lib.name=odbc32 -Pdbtest.endpoint=tcp://localhost:12345 -Pdbtest.endpoint.java=ipc:tsurugi
```

```bash
./gradlew test -Pdbtest.connection.string='Driver={Tsurugi Driver};Endpoint=tcp://localhost:12345;'
```

```bash
./gradlew test -Pdbtest.connection.string='DSN=MyTsurugiDSN;' -Pdbtest.dsn=MyTsurugiDSN
```

- If `-Pdbtest.odbc.lib.name` is omitted, it will be `odbc32`.
- If `-Pdbtest.connection.string` is omitted, it will be `Driver={Tsurugi Driver};Endpoint=%s;`. (`%s` is  value of `-Pdbtest.endpoint`)
- If `-Pdbtest.dsn` is omitted, skip `SQLConnectTest`.
- If `-Pdbtest.endpoint` is omitted, it will be `tcp://localhost:12345`.
- If `-Pdbtest.endpoint.java` is omitted, it is same as value of `-Pdbtest.endpoint`.

### test with credential

```bash
./gradlew test \
-Pdbtest.endpoint=tcp://localhost:12345 \
-Pdbtest.user=user \
-Pdbtest.password=password \
-Pdbtest.auth-token=token \
-Pdbtest.credentials=/path/to/credential-file
```

For tests other than credential, specifying only one of `user`, `auth-token`, or `credentials` is sufficient. If none of these are specified, authentication will be performed using the user `tsurugi`.

In the credential test, anything not specified is skipped.