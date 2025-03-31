#include <stdio.h>
#include "tsubakuro-rust-ffi.h"

/*
The table should be created before execution.

create table customer (
    c_id   bigint primary key,
    c_name varchar(30),
    c_age  int
);
insert into customer values(1, 'Hello', 51);
insert into customer values(2, 'World', 138);
insert into customer values(3, 'Tsurugi', 1);
*/

TsurugiFfiRc example(TsurugiFfiStringHandle endpoint);
TsurugiFfiRc example_connection_option(TsurugiFfiContextHandle context, TsurugiFfiConnectionOptionHandle connection_option, TsurugiFfiStringHandle endpoint);
TsurugiFfiRc example_connect(TsurugiFfiContextHandle context, TsurugiFfiStringHandle endpoint);
TsurugiFfiRc example_sql_client(TsurugiFfiContextHandle context, TsurugiFfiSessionHandle session);
TsurugiFfiRc example_transaction_option(TsurugiFfiContextHandle context, TsurugiFfiTransactionOptionHandle transaction_option);
TsurugiFfiRc example_transaction(TsurugiFfiContextHandle context, TsurugiFfiSqlClientHandle client);
TsurugiFfiRc example_sql(TsurugiFfiContextHandle context, TsurugiFfiSqlClientHandle client, TsurugiFfiTransactionHandle transaction);
TsurugiFfiRc example_statement(TsurugiFfiContextHandle context, TsurugiFfiSqlClientHandle client, TsurugiFfiTransactionHandle transaction);
TsurugiFfiRc example_execute_result(TsurugiFfiContextHandle context, TsurugiFfiSqlExecuteResultHandle execute_result);
TsurugiFfiRc example_query(TsurugiFfiContextHandle context, TsurugiFfiSqlClientHandle client, TsurugiFfiTransactionHandle transaction);
TsurugiFfiRc example_query_result(TsurugiFfiContextHandle context, TsurugiFfiSqlQueryResultHandle query_result);
TsurugiFfiRc example_prepared_statement(TsurugiFfiContextHandle context, TsurugiFfiSqlClientHandle client, TsurugiFfiTransactionHandle transaction);
TsurugiFfiRc example_prepared_execute(TsurugiFfiContextHandle context, TsurugiFfiSqlClientHandle client, TsurugiFfiTransactionHandle transaction, TsurugiFfiSqlPreparedStatementHandle prepared_statement);
TsurugiFfiRc example_prepared_execute_null(TsurugiFfiContextHandle context, TsurugiFfiSqlClientHandle client, TsurugiFfiTransactionHandle transaction, TsurugiFfiSqlPreparedStatementHandle prepared_statement);
TsurugiFfiRc example_prepared_statement_query0(TsurugiFfiContextHandle context, TsurugiFfiSqlClientHandle client, TsurugiFfiTransactionHandle transaction);
TsurugiFfiRc example_prepared_query0(TsurugiFfiContextHandle context, TsurugiFfiSqlClientHandle client, TsurugiFfiTransactionHandle transaction, TsurugiFfiSqlPreparedStatementHandle prepared_statement);
TsurugiFfiRc example_prepared_statement_query1(TsurugiFfiContextHandle context, TsurugiFfiSqlClientHandle client, TsurugiFfiTransactionHandle transaction);
TsurugiFfiRc example_prepared_query1(TsurugiFfiContextHandle context, TsurugiFfiSqlClientHandle client, TsurugiFfiTransactionHandle transaction, TsurugiFfiSqlPreparedStatementHandle prepared_statement);

void main(int argc, char *argv[]) {
    TsurugiFfiStringHandle endpoint;
    if (argc > 1) {
        endpoint = argv[1];
    } else {
        endpoint = "tcp://localhost:12345";
    }
    printf("endpoint=%s\n", endpoint);

    example(endpoint);
}

/*
 * error handling
 *
 * see https://github.com/project-tsurugi/tsubakuro-rust/blob/master/tsubakuro-rust-ffi/src/context.rs
 */
void example_error(TsurugiFfiContextHandle context) {
    if (context == NULL) {
        printf("example_error(): context==NULL\n");
        return;
    }

    TsurugiFfiRc rc;
    tsurugi_ffi_context_get_return_code(context, &rc);
    printf("rc=%x\n", rc);

    TsurugiFfiStringHandle error_name;
    tsurugi_ffi_context_get_error_name(context, &error_name);
    printf("error_name=%s\n", error_name);

    TsurugiFfiStringHandle message;
    tsurugi_ffi_context_get_error_message(context, &message);
    printf("error_message=%s\n", message);

    TsurugiFfiRcType rc_type;
    tsurugi_ffi_context_get_error_type(context, &rc_type);
    if (rc_type == TSURUGI_FFI_RC_TYPE_CORE_SERVER_ERROR) {
        TsurugiFfiStringHandle structured_code;
        tsurugi_ffi_context_get_server_error_structured_code(context, &structured_code);
        printf("structured_code=%s\n", structured_code);
    }
}

/*
 * example main
 */
TsurugiFfiRc example(TsurugiFfiStringHandle endpoint) {
    TsurugiFfiRc rc;

    // enable logger for tusbakuro-rust-ffi
    rc = tsurugi_ffi_env_logger_init();
    if (rc != TSURUGI_FFI_RC_OK) {
        return rc;
    }

    // create context object
    TsurugiFfiContextHandle context;
    rc = tsurugi_ffi_context_create(&context);
    if (rc != TSURUGI_FFI_RC_OK) {
        return rc;
    }

    // execute SQL
    rc = example_connect(context, endpoint);

    // dispose context object
    tsurugi_ffi_context_dispose(context);

    return rc;
}

/*
 * Session connect
 */
TsurugiFfiRc example_connect(TsurugiFfiContextHandle context, TsurugiFfiStringHandle endpoint) {
    TsurugiFfiRc rc;

    // create ConnectionOption
    TsurugiFfiConnectionOptionHandle connection_option;
    rc = tsurugi_ffi_connection_option_create(context, &connection_option);
    if (rc != TSURUGI_FFI_RC_OK) {
        example_error(context);
        return rc;
    }

    rc = example_connection_option(context, connection_option, endpoint);
    if (rc != TSURUGI_FFI_RC_OK) {
        // dispose ConnectionOption
        tsurugi_ffi_connection_option_dispose(connection_option);

        return rc;
    }

    // connect (create Session)
    TsurugiFfiSessionHandle session;
    rc = tsurugi_ffi_session_connect(context, connection_option, &session);
    if (rc != TSURUGI_FFI_RC_OK) {
        example_error(context);

        // dispose ConnectionOption
        tsurugi_ffi_connection_option_dispose(connection_option);

        return rc;
    }

    // dispose ConnectionOption
    tsurugi_ffi_connection_option_dispose(connection_option);

    // execute SQL
    rc = example_sql_client(context, session);

    // dispose SqlClient, Session
    tsurugi_ffi_session_dispose(session);

    return rc;
}

// ConnectionOption
TsurugiFfiRc example_connection_option(TsurugiFfiContextHandle context, TsurugiFfiConnectionOptionHandle connection_option, TsurugiFfiStringHandle endpoint) {
    TsurugiFfiRc rc;

    // set endpoint
    rc = tsurugi_ffi_connection_option_set_endpoint_url(context, connection_option, endpoint);
    if (rc != TSURUGI_FFI_RC_OK) {
        example_error(context);
        return rc;
    }

    // set session label
    rc = tsurugi_ffi_connection_option_set_session_label(context, connection_option, "tsubakuro-rust-ffi/c example session");
    if (rc != TSURUGI_FFI_RC_OK) {
        example_error(context);
        return rc;
    }

    // set default timeout
    TsurugiFfiDuration timeout = 10ull * 1000 * 1000 * 1000; // 10 sec
    rc = tsurugi_ffi_connection_option_set_default_timeout(context, connection_option, timeout);
    if (rc != TSURUGI_FFI_RC_OK) {
        example_error(context);
        return rc;
    }
}

/*
 * SqlClient
 */
TsurugiFfiRc example_sql_client(TsurugiFfiContextHandle context, TsurugiFfiSessionHandle session) {
    TsurugiFfiRc rc;

    // make SqlClient
    TsurugiFfiSqlClientHandle client;
    rc = tsurugi_ffi_session_make_sql_client(context, session, &client);
    if (rc != TSURUGI_FFI_RC_OK) {
        example_error(context);
        return rc;
    }

    // execute SQL
    rc = example_transaction(context, client);

    // dispose SqlClient, Session
    tsurugi_ffi_sql_client_dispose(client);

    return rc;
}

/*
 * Transaction
 */
TsurugiFfiRc example_transaction(TsurugiFfiContextHandle context, TsurugiFfiSqlClientHandle client) {
    TsurugiFfiRc rc;

    // create TransactionOption
    TsurugiFfiTransactionOptionHandle transaction_option;
    rc = tsurugi_ffi_transaction_option_create(context, &transaction_option);
    if (rc != TSURUGI_FFI_RC_OK) {
        example_error(context);
        return rc;
    }

    rc = example_transaction_option(context, transaction_option);
    if (rc != TSURUGI_FFI_RC_OK) {
        // dispose TransactionOption
        tsurugi_ffi_transaction_option_dispose(transaction_option);

        return rc;
    }

    // start transaction (create Transaction)
    TsurugiFfiTransactionHandle transaction;
    rc = tsurugi_ffi_sql_client_start_transaction(context, client, transaction_option, &transaction);
    if (rc != TSURUGI_FFI_RC_OK) {
        example_error(context);

        // dispose TransactionOption
        tsurugi_ffi_transaction_option_dispose(transaction_option);

        return rc;
    }

    // dispose TransactionOption
    tsurugi_ffi_transaction_option_dispose(transaction_option);

    // execute SQL
    rc = example_sql(context, client, transaction);

    // transaction commit
    if (rc == TSURUGI_FFI_RC_OK) {
        // create CommitOption
        TsurugiFfiCommitOptionHandle commit_option;
        rc = tsurugi_ffi_commit_option_create(context, &commit_option);
        if (rc != TSURUGI_FFI_RC_OK) {
            example_error(context);

            // dispose CommitOption, Transaction
            tsurugi_ffi_commit_option_dispose(commit_option);
            tsurugi_ffi_transaction_dispose(transaction);

            return rc;
        }

        // commit
        rc = tsurugi_ffi_sql_client_commit(context, client, transaction, commit_option);
        if (rc != TSURUGI_FFI_RC_OK) {
            example_error(context);
        }

        // dispose CommitOption
        tsurugi_ffi_commit_option_dispose(commit_option);
    }

    // dispose Transaction
    tsurugi_ffi_transaction_dispose(transaction);

    return rc;
}

// TransactionOption
TsurugiFfiRc example_transaction_option(TsurugiFfiContextHandle context, TsurugiFfiTransactionOptionHandle transaction_option) {
    TsurugiFfiRc rc;

    // set transaction type
    rc = tsurugi_ffi_transaction_option_set_transaction_type(context, transaction_option, TSURUGI_FFI_TRANSACTION_TYPE_SHORT);
    if (rc != TSURUGI_FFI_RC_OK) {
        example_error(context);
        return rc;
    }

    // set transaction label
    rc = tsurugi_ffi_transaction_option_set_transaction_label(context, transaction_option, "tsubakuro-rust-ffi/c example transaction");
    if (rc != TSURUGI_FFI_RC_OK) {
        example_error(context);
        return rc;
    }
}

/*
 * execute SQLs
 */
TsurugiFfiRc example_sql(TsurugiFfiContextHandle context, TsurugiFfiSqlClientHandle client, TsurugiFfiTransactionHandle transaction) {
    TsurugiFfiRc rc;

    rc = example_statement(context, client, transaction);
    if (rc != TSURUGI_FFI_RC_OK) {
        return rc;
    }

    rc = example_query(context, client, transaction);
    if (rc != TSURUGI_FFI_RC_OK) {
        return rc;
    }

    rc = example_prepared_statement(context, client, transaction);
    if (rc != TSURUGI_FFI_RC_OK) {
        return rc;
    }

    rc = example_prepared_statement_query0(context, client, transaction);
    if (rc != TSURUGI_FFI_RC_OK) {
        return rc;
    }

    rc = example_prepared_statement_query1(context, client, transaction);
    if (rc != TSURUGI_FFI_RC_OK) {
        return rc;
    }
}

/*
 * execute SQL (update)
 */
TsurugiFfiRc example_statement(TsurugiFfiContextHandle context, TsurugiFfiSqlClientHandle client, TsurugiFfiTransactionHandle transaction) {
    TsurugiFfiRc rc;
    printf("---example_statement---\n");

    TsurugiFfiStringHandle sql = "update customer set c_age = 2 where c_id = 3";
    printf("%s\n", sql);

    TsurugiFfiSqlExecuteResultHandle execute_result;
    rc = tsurugi_ffi_sql_client_execute(context, client, transaction, sql, &execute_result);
    if (rc != TSURUGI_FFI_RC_OK) {
        example_error(context);
        return rc;
    }

    rc = example_execute_result(context, execute_result);

    // dispose SqlExecuteResult
    tsurugi_ffi_sql_execute_result_dispose(execute_result);

    return rc;
}

// SqlExecuteResult
TsurugiFfiRc example_execute_result(TsurugiFfiContextHandle context, TsurugiFfiSqlExecuteResultHandle execute_result) {
    TsurugiFfiRc rc;

    int64_t inserted_rows;
    rc = tsurugi_ffi_sql_execute_result_get_inserted_rows(context, execute_result, &inserted_rows);
    if (rc != TSURUGI_FFI_RC_OK) {
        example_error(context);
        return rc;
    }
    printf("inserted_rows=%ld\n", inserted_rows);

    int64_t updated_rows;
    rc = tsurugi_ffi_sql_execute_result_get_updated_rows(context, execute_result, &updated_rows);
    if (rc != TSURUGI_FFI_RC_OK) {
        example_error(context);
        return rc;
    }
    printf("updated_rows=%ld\n", updated_rows);

    int64_t merged_rows;
    rc = tsurugi_ffi_sql_execute_result_get_merged_rows(context, execute_result, &merged_rows);
    if (rc != TSURUGI_FFI_RC_OK) {
        example_error(context);
        return rc;
    }
    printf("merged_rows=%ld\n", merged_rows);

    int64_t deleted_rows;
    rc = tsurugi_ffi_sql_execute_result_get_deleted_rows(context, execute_result, &deleted_rows);
    if (rc != TSURUGI_FFI_RC_OK) {
        example_error(context);
        return rc;
    }
    printf("deleted_rows=%ld\n", deleted_rows);

    return rc;
}

/*
 * execute SQL (select)
 */
TsurugiFfiRc example_query(TsurugiFfiContextHandle context, TsurugiFfiSqlClientHandle client, TsurugiFfiTransactionHandle transaction) {
    TsurugiFfiRc rc;
    printf("---example_query---\n");

    TsurugiFfiStringHandle sql = "select c_id, c_name, c_age from customer order by c_id";
    printf("%s\n", sql);

    TsurugiFfiSqlQueryResultHandle query_result;
    rc = tsurugi_ffi_sql_client_query(context, client, transaction, sql, &query_result);
    if (rc != TSURUGI_FFI_RC_OK) {
        example_error(context);
        return rc;
    }

    rc = example_query_result(context, query_result);

    // dispose SqlQueryResult
    tsurugi_ffi_sql_query_result_dispose(query_result);

    return rc;
}

// SqlQueryResult
TsurugiFfiRc example_query_result(TsurugiFfiContextHandle context, TsurugiFfiSqlQueryResultHandle query_result) {
    TsurugiFfiRc rc;

    for (;;) {
        bool next_row;
        rc = tsurugi_ffi_sql_query_result_next_row(context, query_result, &next_row);
        if (rc != TSURUGI_FFI_RC_OK) {
            example_error(context);
            return rc;
        }
        if (!next_row) {
            break;
        }

        // fetch c_id (bigint not null)
        bool next_column;
        rc = tsurugi_ffi_sql_query_result_next_column(context, query_result, &next_column);
        if (rc != TSURUGI_FFI_RC_OK) {
            example_error(context);
            return rc;
        }
        if (next_column) {
            int64_t value;
            rc = tsurugi_ffi_sql_query_result_fetch_int8(context, query_result, &value);
            if (rc != TSURUGI_FFI_RC_OK) {
                example_error(context);
                return rc;
            }
            printf("c_id=%ld\n", value);
        }

        // fetch c_name (varchar)
        rc = tsurugi_ffi_sql_query_result_next_column(context, query_result, &next_column);
        if (rc != TSURUGI_FFI_RC_OK) {
            example_error(context);
            return rc;
        }
        if (next_column) {
            bool is_null;
            rc = tsurugi_ffi_sql_query_result_is_null(context, query_result, &is_null);
            if (rc != TSURUGI_FFI_RC_OK) {
                example_error(context);
                return rc;
            }
            if (!is_null) {
                TsurugiFfiStringHandle value;
                rc = tsurugi_ffi_sql_query_result_fetch_character(context, query_result, &value);
                if (rc != TSURUGI_FFI_RC_OK) {
                    example_error(context);
                    return rc;
                }
                printf("c_name=%s\n", value);
            }
        }

        // fetch c_age (int)
        rc = tsurugi_ffi_sql_query_result_next_column(context, query_result, &next_column);
        if (rc != TSURUGI_FFI_RC_OK) {
            example_error(context);
            return rc;
        }
        if (next_column) {
            bool is_null;
            rc = tsurugi_ffi_sql_query_result_is_null(context, query_result, &is_null);
            if (rc != TSURUGI_FFI_RC_OK) {
                example_error(context);
                return rc;
            }
            if (!is_null) {
                int32_t value;
                rc = tsurugi_ffi_sql_query_result_fetch_int4(context, query_result, &value);
                if (rc != TSURUGI_FFI_RC_OK) {
                    example_error(context);
                    return rc;
                }
                printf("c_age=%d\n", value);
            }
        }
    } // end loop

    return rc;
}

/*
 * execute prepared SQL (insert)
 */
TsurugiFfiRc example_prepared_statement(TsurugiFfiContextHandle context, TsurugiFfiSqlClientHandle client, TsurugiFfiTransactionHandle transaction) {
    TsurugiFfiRc rc;
    printf("---example_prepared_statement---\n");

    TsurugiFfiStringHandle sql = "insert or replace into customer values(:id, :name, :age)";
    printf("%s\n", sql);

    TsurugiFfiSqlPlaceholderHandle p0;
    rc = tsurugi_ffi_sql_placeholder_of_atom_type(context, "id", TSURUGI_FFI_ATOM_TYPE_INT8, &p0);
    if (rc != TSURUGI_FFI_RC_OK) {
        example_error(context);
        return rc;
    }

    TsurugiFfiSqlPlaceholderHandle p1;
    rc = tsurugi_ffi_sql_placeholder_of_atom_type(context, "name", TSURUGI_FFI_ATOM_TYPE_CHARACTER, &p1);
    if (rc != TSURUGI_FFI_RC_OK) {
        example_error(context);

        tsurugi_ffi_sql_placeholder_dispose(p0);
        return rc;
    }

    TsurugiFfiSqlPlaceholderHandle p2;
    rc = tsurugi_ffi_sql_placeholder_of_atom_type(context, "age", TSURUGI_FFI_ATOM_TYPE_INT4, &p2);
    if (rc != TSURUGI_FFI_RC_OK) {
        example_error(context);

        tsurugi_ffi_sql_placeholder_dispose(p0);
        tsurugi_ffi_sql_placeholder_dispose(p1);
        return rc;
    }

    TsurugiFfiSqlPlaceholderHandle placeholders[] = { p0, p1, p2 };
    uint32_t placeholders_size = 3;
    TsurugiFfiSqlPreparedStatementHandle prepared_statement;
    rc = tsurugi_ffi_sql_client_prepare(context, client, sql, placeholders, placeholders_size, &prepared_statement);
    if (rc != TSURUGI_FFI_RC_OK) {
        example_error(context);

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
    if (rc == TSURUGI_FFI_RC_OK) {
        rc = example_prepared_execute_null(context, client, transaction, prepared_statement);
    }

    // dispose SqlPreparedStatement
    tsurugi_ffi_sql_prepared_statement_dispose(prepared_statement);

    return rc;
}

// insert ... values(4, 'example', 20)
TsurugiFfiRc example_prepared_execute(TsurugiFfiContextHandle context, TsurugiFfiSqlClientHandle client, TsurugiFfiTransactionHandle transaction, TsurugiFfiSqlPreparedStatementHandle prepared_statement) {
    TsurugiFfiRc rc;

    int64_t id = 4;
    TsurugiFfiSqlParameterHandle p0;
    rc = tsurugi_ffi_sql_parameter_of_int8(context, "id", id, &p0);
    if (rc != TSURUGI_FFI_RC_OK) {
        example_error(context);
        return rc;
    }

    TsurugiFfiStringHandle name = "example";
    TsurugiFfiSqlParameterHandle p1;
    rc = tsurugi_ffi_sql_parameter_of_character(context, "name", name, &p1);
    if (rc != TSURUGI_FFI_RC_OK) {
        example_error(context);

        tsurugi_ffi_sql_parameter_dispose(p0);
        return rc;
    }

    int32_t age = 20;
    TsurugiFfiSqlParameterHandle p2;
    rc = tsurugi_ffi_sql_parameter_of_int4(context, "age", age, &p2);
    if (rc != TSURUGI_FFI_RC_OK) {
        example_error(context);

        tsurugi_ffi_sql_parameter_dispose(p0);
        tsurugi_ffi_sql_parameter_dispose(p1);
        return rc;
    }

    TsurugiFfiSqlParameterHandle parameters[] = { p0, p1, p2 };
    uint32_t parameters_size = 3;
    TsurugiFfiSqlExecuteResultHandle execute_result;
    rc = tsurugi_ffi_sql_client_prepared_execute(context, client, transaction, prepared_statement, parameters, parameters_size, &execute_result);
    if (rc != TSURUGI_FFI_RC_OK) {
        example_error(context);

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

// insert ... values(9, null, null)
TsurugiFfiRc example_prepared_execute_null(TsurugiFfiContextHandle context, TsurugiFfiSqlClientHandle client, TsurugiFfiTransactionHandle transaction, TsurugiFfiSqlPreparedStatementHandle prepared_statement) {
    TsurugiFfiRc rc;

    int64_t id = 9;
    TsurugiFfiSqlParameterHandle p0;
    rc = tsurugi_ffi_sql_parameter_of_int8(context, "id", id, &p0);
    if (rc != TSURUGI_FFI_RC_OK) {
        example_error(context);
        return rc;
    }

    TsurugiFfiSqlParameterHandle p1;
    rc = tsurugi_ffi_sql_parameter_null(context, "name", &p1);
    if (rc != TSURUGI_FFI_RC_OK) {
        example_error(context);

        tsurugi_ffi_sql_parameter_dispose(p0);
        return rc;
    }

    TsurugiFfiSqlParameterHandle p2;
    rc = tsurugi_ffi_sql_parameter_null(context, "age", &p2);
    if (rc != TSURUGI_FFI_RC_OK) {
        example_error(context);

        tsurugi_ffi_sql_parameter_dispose(p0);
        tsurugi_ffi_sql_parameter_dispose(p1);
        return rc;
    }

    TsurugiFfiSqlParameterHandle parameters[] = { p0, p1, p2 };
    uint32_t parameters_size = 3;
    TsurugiFfiSqlExecuteResultHandle execute_result;
    rc = tsurugi_ffi_sql_client_prepared_execute(context, client, transaction, prepared_statement, parameters, parameters_size, &execute_result);
    if (rc != TSURUGI_FFI_RC_OK) {
        example_error(context);

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

/*
 * execute prepared SQL (select)
 */
TsurugiFfiRc example_prepared_statement_query0(TsurugiFfiContextHandle context, TsurugiFfiSqlClientHandle client, TsurugiFfiTransactionHandle transaction) {
    TsurugiFfiRc rc;
    printf("---example_prepared_statement_query0---\n");

    TsurugiFfiStringHandle sql = "select * from customer order by c_id";
    printf("%s\n", sql);

    TsurugiFfiSqlPlaceholderHandle placeholders[] = {};
    uint32_t placeholders_size = 0;
    TsurugiFfiSqlPreparedStatementHandle prepared_statement;
    rc = tsurugi_ffi_sql_client_prepare(context, client, sql, placeholders, placeholders_size, &prepared_statement);
    if (rc != TSURUGI_FFI_RC_OK) {
        example_error(context);
        return rc;
    }

    // execute SQL
    rc = example_prepared_query0(context, client, transaction, prepared_statement);

    // dispose SqlPreparedStatement
    tsurugi_ffi_sql_prepared_statement_dispose(prepared_statement);

    return rc;
}

TsurugiFfiRc example_prepared_query0(TsurugiFfiContextHandle context, TsurugiFfiSqlClientHandle client, TsurugiFfiTransactionHandle transaction, TsurugiFfiSqlPreparedStatementHandle prepared_statement) {
    TsurugiFfiRc rc;

    TsurugiFfiSqlParameterHandle parameters[] = {};
    uint32_t parameters_size = 0;
    TsurugiFfiSqlQueryResultHandle query_result;
    rc = tsurugi_ffi_sql_client_prepared_query(context, client, transaction, prepared_statement, parameters, parameters_size, &query_result);
    if (rc != TSURUGI_FFI_RC_OK) {
        example_error(context);
        return rc;
    }

    rc = example_query_result(context, query_result);

    // dispose SqlQueryResult
    tsurugi_ffi_sql_query_result_dispose(query_result);

    return rc;
}

/*
 * execute prepared SQL (select)
 */
TsurugiFfiRc example_prepared_statement_query1(TsurugiFfiContextHandle context, TsurugiFfiSqlClientHandle client, TsurugiFfiTransactionHandle transaction) {
    TsurugiFfiRc rc;
    printf("---example_prepared_statement_query1---\n");

    TsurugiFfiStringHandle sql = "select * from customer where c_id = :id";
    printf("%s\n", sql);

    TsurugiFfiSqlPlaceholderHandle p0;
    rc = tsurugi_ffi_sql_placeholder_of_atom_type(context, "id", TSURUGI_FFI_ATOM_TYPE_INT8, &p0);
    if (rc != TSURUGI_FFI_RC_OK) {
        example_error(context);
        return rc;
    }

    TsurugiFfiSqlPlaceholderHandle placeholders[] = { p0 };
    uint32_t placeholders_size = 1;
    TsurugiFfiSqlPreparedStatementHandle prepared_statement;
    rc = tsurugi_ffi_sql_client_prepare(context, client, sql, placeholders, placeholders_size, &prepared_statement);
    if (rc != TSURUGI_FFI_RC_OK) {
        example_error(context);

        tsurugi_ffi_sql_placeholder_dispose(p0);
        return rc;
    }

    // dispose SqlPlaceholder
    tsurugi_ffi_sql_placeholder_dispose(p0);

    // execute SQL
    rc = example_prepared_query1(context, client, transaction, prepared_statement);

    // dispose SqlPreparedStatement
    tsurugi_ffi_sql_prepared_statement_dispose(prepared_statement);

    return rc;
}

TsurugiFfiRc example_prepared_query1(TsurugiFfiContextHandle context, TsurugiFfiSqlClientHandle client, TsurugiFfiTransactionHandle transaction, TsurugiFfiSqlPreparedStatementHandle prepared_statement) {
    TsurugiFfiRc rc;

    int64_t id = 3;
    TsurugiFfiSqlParameterHandle p0;
    rc = tsurugi_ffi_sql_parameter_of_int8(context, "id", id, &p0);
    if (rc != TSURUGI_FFI_RC_OK) {
        example_error(context);
        return rc;
    }

    TsurugiFfiSqlParameterHandle parameters[] = { p0 };
    uint32_t parameters_size = 1;
    TsurugiFfiSqlQueryResultHandle query_result;
    rc = tsurugi_ffi_sql_client_prepared_query(context, client, transaction, prepared_statement, parameters, parameters_size, &query_result);
    if (rc != TSURUGI_FFI_RC_OK) {
        example_error(context);

        tsurugi_ffi_sql_parameter_dispose(p0);
        return rc;
    }

    // dispose SqlParameter
    tsurugi_ffi_sql_parameter_dispose(p0);

    rc = example_query_result(context, query_result);

    // dispose SqlQueryResult
    tsurugi_ffi_sql_query_result_dispose(query_result);

    return rc;
}
