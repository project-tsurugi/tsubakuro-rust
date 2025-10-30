# tsubakuro-rust-ffi

tsubakuro-rust-ffi is a library providing functions in C ABI format to access [Tsurugi](https://github.com/project-tsurugi/tsurugidb).

tsubakuro-rust-ffi uses [tsubakuro-rust-core](../tsubakuro-rust-core), so refer to that for explanation.

## Target

- Tsurugi 1.7.0 or later.

## How to build

Since it depends on [tsubakuro-rust-core](../tsubakuro-rust-core), its build environment (e.g. `protoc`) is required.

```bash
cd tsubakuro-rust-ffi
cargo build --release
ls target/release/*tsubakuro_rust_ffi*
```

`libtsubakuro_rust_ffi.so` (`tsubakuro_rust_ffi.dll` for MS-Windows) is generated.

## How to generate C header file

```bash
cd tsubakuro-rust-ffi
cbindgen --lang C --output tsubakuro-rust-ffi.h
```

- [cbindgen](https://github.com/mozilla/cbindgen)

There is an example in [tsubakuro-rust-java](../tsubakuro-rust-java).

## Example

### initialize example

```c
typedef uint32_t TsurugiFfiRc;

TsurugiFfiRc example() {
    TsurugiFfiRc rc;

    // enable logger for tusbakuro-rust-ffi
    rc = tsurugi_ffi_env_logger_init();
    if (rc != 0) {
        return rc;
    }

    // create context object
    TsurugiFfiContextHandle context;
    rc = tsurugi_ffi_context_create(&context);
    if (rc != 0) {
        return rc;
    }

    // execute SQL
    rc = example_connect(context);

    // dispose context object
    tsurugi_ffi_context_dispose(context);

    return rc;    
}
```

### connect example

```c
TsurugiFfiRc example_connect(TsurugiFfiContextHandle context) {
    TsurugiFfiRc rc;

    // create ConnectionOption
    TsurugiFfiConnectionOptionHandle connection_option;
    rc = tsurugi_ffi_connection_option_create(context, &connection_option);
    if (rc != 0) {
        // error handling
        TsurugiFfiStringHandle message;
        tsurugi_ffi_context_get_error_message(context, &message);
        printf("%s\n", message);
        return rc;
    }

    // set endpoint
    rc = tsurugi_ffi_connection_option_set_endpoint_url(context, connection_option, "tcp://localhost:12345");
    if (rc != 0) {
        ～error handling～

        // dispose ConnectionOption
        tsurugi_ffi_connection_option_dispose(connection_option);

        return rc;
    }

    // create Credential
    TsurugiFfiCredentialHandle credential;
    rc = tsurugi_ffi_credential_from_user_password(context, "user", "password", &credential);
    if (rc != TSURUGI_FFI_RC_OK) {
        ～error handling～

        // dispose ConnectionOption
        tsurugi_ffi_connection_option_dispose(connection_option);
        return rc;
    }

    // set Credential
    rc = tsurugi_ffi_connection_option_set_credential(context, connection_option, credential);
    if (rc != TSURUGI_FFI_RC_OK) {
        ～error handling～

        // dispose Credential, ConnectionOption
        tsurugi_ffi_credential_dispose(credential);
        tsurugi_ffi_connection_option_dispose(connection_option);
        return rc;
    }

    // dispose Credential
    tsurugi_ffi_credential_dispose(credential);

    // connect (create Session)
    TsurugiFfiSessionHandle session;
    rc = tsurugi_ffi_session_connect(context, connection_option, &session);
    if (rc != 0) {
        ～error handling～

        // dispose ConnectionOption
        tsurugi_ffi_connection_option_dispose(connection_option);

        return rc;
    }

    // dispose ConnectionOption
    tsurugi_ffi_connection_option_dispose(connection_option);

    // make SqlClient
    TsurugiFfiSqlClientHandle client;
    rc = tsurugi_ffi_session_make_sql_client(context, session, &client);
    if (rc != 0) {
        ～error handling～

        // dispose Session
        tsurugi_ffi_session_dispose(session);

        return rc;
    }

    // execute SQL
    rc = example_transaction(context, client);

    // dispose SqlClient, Session
    tsurugi_ffi_sql_client_dispose(client);
    tsurugi_ffi_session_dispose(session);

    return rc;
}
```

See [example/c](example/c/) for more examples.

## License

[Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)

