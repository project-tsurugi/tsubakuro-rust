# tsubakuro-rust-ffi

tsubakuro-rust-ffi is a library providing functions in C ABI format. (Functions called from other programming languages)

Depends on [tsubakuro-rust-core](../tsubakuro-rust-core).

## How to build

```bash
cd tsubakuro-rust-ffi
cargo build --release
ls target/release/
```

## How to generate C header file

```bash
cd tsubakuro-rust-ffi
cbindgen --lang C --output tsubakuro-rust-ffi.h
```

- [cbindgen](https://github.com/mozilla/cbindgen)

There is an example in [tsubakuro-rust-java](../tsubakuro-rust-java).

## Example

In the following example, error handling and the call to dispose when an error occurs are omitted.

```c
typedef uint32_t TsurugiFfiRc;

// logger init
TsurugiFfiRc rc = tsurugi_ffi_env_logger_init();
if (rc != 0) {
    return rc;
}

// create context
TsurugiFfiContextHandle context;
rc = tsurugi_ffi_context_create(&context);
if (rc != 0) {
    return rc;
}

// connect
TsurugiFfiConnectionOptionHandle connection_option;
rc = tsurugi_ffi_connection_option_create(context, &connection_option);
if (rc != 0) {
    const char *message;
    tsurugi_ffi_context_get_error_message(context, &message);
    printf("%s\n", message);
    return rc;
}
rc = tsurugi_ffi_connection_option_set_endpoint_url(context, connection_option, "tcp://localhost:12345");
if (rc != 0) {
    ～
    return rc;
}

TsurugiFfiSessionHandle session;
rc = tsurugi_ffi_session_connect(context, connection_option, &session);
if (rc != 0) {
    ～
    return rc;
}
tsurugi_ffi_connection_option_dispose(connection_option);

// make SqlClient
TsurugiFfiSqlClientHandle client;
rc = tsurugi_ffi_session_make_sql_client(context, session, &client);
if (rc != 0) {
    ～
    return rc;
}

～
    
// dispose
tsurugi_ffi_sql_client_dispose(client);
tsurugi_ffi_session_dispose(session);
tsurugi_ffi_context_dispose(context);
```

