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
        const char *message;
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

### transaction example

```c
TsurugiFfiRc example_connect(TsurugiFfiContextHandle context, TsurugiFfiSqlClientHandle client) {
    TsurugiFfiRc rc;

    // create TransactionOption
    TsurugiFfiTransactionOptionHandle transaction_option;
    rc = tsurugi_ffi_transaction_option_create(context, &transaction_option);
    if (rc != 0) {
        ～error handling～
        return rc;
    }

    // set transaction type
    rc = tsurugi_ffi_transaction_option_set_transaction_type(context, transaction_option, TSURUGI_FFI_TRANSACTION_TYPE_SHORT);
    if (rc != 0) {
        ～error handling～

        // dispose TransactionOption
        tsurugi_ffi_transaction_option_dispose(transaction_option);

        return rc;
    }

    // start transaction (create Transaction)
    TsurugiFfiTransactionHandle transaction;
    rc = tsurugi_ffi_sql_client_start_transaction(context, client, transaction_option, &transaction);
    if (rc != 0) {
        ～error handling～

        // dispose TransactionOption
        tsurugi_ffi_transaction_option_dispose(transaction_option);

        return rc;
    }

    // dispose TransactionOption
    tsurugi_ffi_transaction_option_dispose(transaction_option);

    // execute SQL
    rc = example_sql(context, client, transaction);

    // transaction commit
    if (rc == 0) {
        // create CommitOption
        TsurugiFfiCommitOptionHandle commit_option;
        rc = tsurugi_ffi_commit_option_create(context, &commit_option);
        if (rc != 0) {
            ～error handling～

            // dispose CommitOption, Transaction
            tsurugi_ffi_commit_option_dispose(commit_option);
            tsurugi_ffi_transaction_dispose(transaction);

            return rc;
        }

        // commit
        rc = tsurugi_ffi_sql_client_commit(context, client, transction, commit_option);
        if (rc != 0) {
            ～error handling～
        }

        // dispose CommitOption
        tsurugi_ffi_commit_option_dispose(commit_option);
    }

    // dispose Transaction
    tsurugi_ffi_transaction_dispose(transaction);

    return rc;
}
```

### execute SQL(update) example

```c
TsurugiFfiRc example_statement(TsurugiFfiContextHandle context, TsurugiFfiSqlClientHandle client, TsurugiFfiTransactionHandle transaction) {
    TsurugiFfiRc rc;

    const char *sql = "update customer set c_age = 2 where c_id = 3";

    TsurugiFfiSqlExecuteResultHandle execute_result;
    rc = tsurugi_ffi_sql_client_execute(context, client, transction, sql);
    if (rc != 0) {
        ～error handling～
        return rc;
    }

    // get rows
    int64_t updated_rows;
    rc = tsurugi_ffi_sql_execute_result_get_updated_rows(context, execute_result, &updated_rows);
    if (rc != 0) {
        ～error handling～

        // dispose SqlExecuteResult
        tsurugi_ffi_sql_execute_result_dispose(execute_result);

        return rc;
    }

    printf("undated_rows=%ld\n", updated_rows);

    // dispose SqlExecuteResult
    tsurugi_ffi_sql_execute_result_dispose(execute_result);

    return rc;
}
```

