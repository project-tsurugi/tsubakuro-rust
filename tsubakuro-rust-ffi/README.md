# tsubakuro-rust-ffi

tsubakuro-rust-ffi is a library providing functions in C ABI format to access [Tsurugi](https://github.com/project-tsurugi/tsurugidb).

tsubakuro-rust-ffi uses [tsubakuro-rust-core](../tsubakuro-rust-core), so refer to that for explanation.

## How to build

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
TsurugiFfiRc example_transaction(TsurugiFfiContextHandle context, TsurugiFfiSqlClientHandle client) {
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
        rc = tsurugi_ffi_sql_client_commit(context, client, transaction, commit_option);
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

    TsurugiFfiStringHandle sql = "update customer set c_age = 2 where c_id = 3";

    TsurugiFfiSqlExecuteResultHandle execute_result;
    rc = tsurugi_ffi_sql_client_execute(context, client, transaction, sql, &execute_result);
    if (rc != 0) {
        ～error handling～
        return rc;
    }

    rc = example_execute_result(context, execute_result);

    // dispose SqlExecuteResult
    tsurugi_ffi_sql_execute_result_dispose(execute_result);

    return rc;
}

TsurugiFfiRc example_execute_result(TsurugiFfiContextHandle context, TsurugiFfiSqlExecuteResultHandle execute_result) {
    TsurugiFfiRc rc;

    // get rows
    int64_t updated_rows;
    rc = tsurugi_ffi_sql_execute_result_get_updated_rows(context, execute_result, &updated_rows);
    if (rc != 0) {
        ～error handling～
        return rc;
    }

    printf("updated_rows=%ld\n", updated_rows);

    return rc;
}
```

### execute SQL(select) example

```c
TsurugiFfiRc example_query(TsurugiFfiContextHandle context, TsurugiFfiSqlClientHandle client, TsurugiFfiTransactionHandle transaction) {
    TsurugiFfiRc rc;

    TsurugiFfiStringHandle sql = "select c_id, c_name, c_age from customer order by c_id";

    TsurugiFfiSqlQueryResultHandle query_result;
    rc = tsurugi_ffi_sql_client_query(context, client, transaction, sql, &query_result);
    if (rc != 0) {
        ～error handling～
        return rc;
    }

    rc = example_query_result(context, query_result);

    // dispose SqlQueryResult
    tsurugi_ffi_sql_query_result_dispose(query_result);

    return rc;
}

TsurugiFfiRc example_query_result(TsurugiFfiContextHandle context, TsurugiFfiSqlQueryResultHandle query_result) {
    TsurugiFfiRc rc;

    for (;;) {
        bool next_row;
        rc = tsurugi_ffi_sql_query_result_next_row(context, query_result, &next_row);
        if (rc != 0) {
            ～error handling～
            return rc;
        }
        if (!next_row) {
            break;
        }

        // fetch c_id (bigint not null)
        bool next_column;
        rc = tsurugi_ffi_sql_query_result_next_column(context, query_result, &next_column);
        if (rc != 0) {
            ～error handling～
            return rc;
        }
        if (next_column) {
            int64_t value;
            rc = tsurugi_ffi_sql_query_result_fetch_int8(context, query_result, &value);
            if (rc != 0) {
                ～error handling～
                return rc;
            }
            printf("c_id=%ld\n", value);
        }

        // fetch c_name (varchar)
        rc = tsurugi_ffi_sql_query_result_next_column(context, query_result, &next_column);
        if (rc != 0) {
            ～error handling～
            return rc;
        }
        if (next_column) {
            bool is_null;
            rc = tsurugi_ffi_sql_query_result_is_null(context, query_result, &is_null);
            if (rc != 0) {
                ～error handling～
                return rc;
            }
            if (!is_null) {
                TsurugiFfiStringHandle value;
                rc = tsurugi_ffi_sql_query_result_fetch_character(context, query_result, &value);
                if (rc != 0) {
                    ～error handling～
                    return rc;
                }
                printf("c_name=%s\n", value);
            }
        }

        // fetch c_age (int)
        rc = tsurugi_ffi_sql_query_result_next_column(context, query_result, &next_column);
        if (rc != 0) {
            ～error handling～
            return rc;
        }
        if (next_column) {
            bool is_null;
            rc = tsurugi_ffi_sql_query_result_is_null(context, query_result, &is_null);
            if (rc != 0) {
                ～error handling～
                return rc;
            }
            if (!is_null) {
                int32_t value;
                rc = tsurugi_ffi_sql_query_result_fetch_int4(context, query_result, &value);
                if (rc != 0) {
                    ～error handling～
                    return rc;
                }
                printf("c_age=%d\n", value);
            }
        }
    } // end loop

    return rc;
}
```

#### prepared statement(insert) example

```c
TsurugiFfiRc example_prepared_statement_insert(TsurugiFfiContextHandle context, TsurugiFfiSqlClientHandle client, TsurugiFfiTransactionHandle transaction) {
    TsurugiFfiRc rc;

    TsurugiFfiStringHandle sql = "insert into customer values(:id, :name, :age)";

    TsurugiFfiSqlPlaceholderHandle p0;
    rc = tsurugi_ffi_sql_placeholder_of_atom_type(context, "id", TSURUGI_FFI_ATOM_TYPE_INT8, &p0);
    if (rc != 0) {
        ～error handling～
        return rc;
    }

    TsurugiFfiSqlPlaceholderHandle p1;
    rc = tsurugi_ffi_sql_placeholder_of_atom_type(context, "name", TSURUGI_FFI_ATOM_TYPE_CHARACTER, &p1);
    if (rc != 0) {
        ～error handling～

        tsurugi_ffi_sql_placeholder_dispose(p0);
        return rc;
    }

    TsurugiFfiSqlPlaceholderHandle p2;
    rc = tsurugi_ffi_sql_placeholder_of_atom_type(context, "age", TSURUGI_FFI_ATOM_TYPE_INT4, &p2);
    if (rc != 0) {
        ～error handling～

        tsurugi_ffi_sql_placeholder_dispose(p0);
        tsurugi_ffi_sql_placeholder_dispose(p1);
        return rc;
    }

    TsurugiFfiSqlPlaceholderHandle placeholders[] = { p0, p1, p2 };
    uint32_t placeholders_size = 3;
    TsurugiFfiSqlPreparedStatementHandle prepared_statement;
    rc = tsurugi_ffi_sql_client_prepare(context, client, sql, placeholders, placeholders_size, &prepared_statement);
    if (rc != 0) {
        ～error handling～

        tsurugi_ffi_sql_placeholder_dispose(p0);
        tsurugi_ffi_sql_placeholder_dispose(p1);
        tsurugi_ffi_sql_placeholder_dispose(p2);
        return rc;
    }

    // dispose SqlPlaceholder
    tsurugi_ffi_sql_placeholder_dispose(p0);
    tsurugi_ffi_sql_placeholder_dispose(p1);
    tsurugi_ffi_sql_placeholder_dispose(p2);

    // execute SQL
    rc = example_prepared_execute(context, client, transaction, prepared_statement);

    // dispose SqlPreparedStatement
    tsurugi_ffi_sql_prepared_statement_dispose(prepared_statement);

    return rc;
}

TsurugiFfiRc example_prepared_execute(TsurugiFfiContextHandle context, TsurugiFfiSqlClientHandle client, TsurugiFfiTransactionHandle transaction, TsurugiFfiSqlPreparedStatementHandle prepared_statement) {
    TsurugiFfiRc rc;

    int64_t id = 4;
    TsurugiFfiSqlParameterHandle p0;
    rc = tsurugi_ffi_sql_parameter_of_int8(context, "id", id, &p0);
    if (rc != 0) {
        ～error handling～
        return rc;
    }

    TsurugiFfiStringHandle name = "example";
    TsurugiFfiSqlParameterHandle p1;
    rc = tsurugi_ffi_sql_parameter_of_character(context, "name", name, &p1);
    if (rc != 0) {
        ～error handling～

        tsurugi_ffi_sql_parameter_dispose(p0);
        return rc;
    }

    int32_t age = 20;
    TsurugiFfiSqlParameterHandle p2;
    rc = tsurugi_ffi_sql_parameter_of_int4(context, "age", age, &p2);
    if (rc != 0) {
        ～error handling～

        tsurugi_ffi_sql_parameter_dispose(p0);
        tsurugi_ffi_sql_parameter_dispose(p1);
        return rc;
    }

    TsurugiFfiSqlParameterHandle parameters[] = { p0, p1, p2 };
    uint32_t parameters_size = 3;
    TsurugiFfiSqlExecuteResultHandle execute_result;
    rc = tsurugi_ffi_sql_client_prepared_execute(context, client, transaction, prepared_statement, parameters, parameters_size, &execute_result);
    if (rc != 0) {
        ～error handling～

        tsurugi_ffi_sql_parameter_dispose(p0);
        tsurugi_ffi_sql_parameter_dispose(p1);
        tsurugi_ffi_sql_parameter_dispose(p2);
        return rc;
    }

    // dispose SqlParameter
    tsurugi_ffi_sql_parameter_dispose(p0);
    tsurugi_ffi_sql_parameter_dispose(p1);
    tsurugi_ffi_sql_parameter_dispose(p2);

    rc = example_execute_result(context, execute_result);

    // dispose SqlExecuteResult
    tsurugi_ffi_sql_execute_result_dispose(execute_result);

    return rc;
}
```

## License

[Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)

