package com.tsurugidb.tsubakuro.rust.java.service.sql;

import java.lang.foreign.MemorySegment;
import java.nio.file.Path;
import java.time.Duration;
import java.util.List;
import java.util.Objects;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.job.TgFfiJob;
import com.tsurugidb.tsubakuro.rust.java.job.TgFfiVoidJob;
import com.tsurugidb.tsubakuro.rust.java.rc.TgFfiRcUtil;
import com.tsurugidb.tsubakuro.rust.java.service.sql.prepare.TgFfiSqlParameter;
import com.tsurugidb.tsubakuro.rust.java.service.sql.prepare.TgFfiSqlPlaceholder;
import com.tsurugidb.tsubakuro.rust.java.service.sql.prepare.TgFfiSqlPreparedStatement;
import com.tsurugidb.tsubakuro.rust.java.service.sql.type.TgFfiBlobReference;
import com.tsurugidb.tsubakuro.rust.java.service.sql.type.TgFfiClobReference;
import com.tsurugidb.tsubakuro.rust.java.transaction.TgFfiCommitOption;
import com.tsurugidb.tsubakuro.rust.java.transaction.TgFfiTransaction;
import com.tsurugidb.tsubakuro.rust.java.transaction.TgFfiTransactionOption;
import com.tsurugidb.tsubakuro.rust.java.transaction.TgFfiTransactionStatus;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObject;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObjectManager;

public class TgFfiSqlClient extends TgFfiObject {

    public TgFfiSqlClient(TgFfiObjectManager manager, MemorySegment handle) {
        super(manager, handle);
    }

    public synchronized String getServiceMessageVersion(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var out = allocatePtrOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_get_service_message_version(ctx, handle, out);
        TgFfiRcUtil.throwIfError(rc, context);

        return outToString(out);
    }

    public synchronized TgFfiTableList listTables(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var out = allocateHandleOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_list_tables(ctx, handle, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiTableList(manager(), outHandle);
    }

    public synchronized TgFfiTableList listTablesFor(TgFfiContext context, Duration timeout) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var t = allocateDuration(timeout);
        var out = allocateHandleOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_list_tables_for(ctx, handle, t, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiTableList(manager(), outHandle);
    }

    public synchronized TgFfiJob<TgFfiTableList> listTablesAsync(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var out = allocateHandleOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_list_tables_async(ctx, handle, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiJob<>(manager(), outHandle) {
            @Override
            protected TgFfiTableList valueToFfiObject(TgFfiObjectManager manager, MemorySegment valueHandle) {
                return new TgFfiTableList(manager, valueHandle);
            }
        };
    }

    public synchronized TgFfiTableMetadata getTableMetadata(TgFfiContext context, String tableName) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var arg = allocateString(tableName);
        var out = allocateHandleOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_get_table_metadata(ctx, handle, arg, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiTableMetadata(manager(), outHandle);
    }

    public synchronized TgFfiTableMetadata getTableMetadataFor(TgFfiContext context, String tableName, Duration timeout) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var arg = allocateString(tableName);
        var t = allocateDuration(timeout);
        var out = allocateHandleOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_get_table_metadata_for(ctx, handle, arg, t, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiTableMetadata(manager(), outHandle);
    }

    public synchronized TgFfiJob<TgFfiTableMetadata> getTableMetadataAsync(TgFfiContext context, String tableName) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var arg = allocateString(tableName);
        var out = allocateHandleOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_get_table_metadata_async(ctx, handle, arg, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiJob<>(manager(), outHandle) {
            @Override
            protected TgFfiTableMetadata valueToFfiObject(TgFfiObjectManager manager, MemorySegment valueHandle) {
                return new TgFfiTableMetadata(manager, valueHandle);
            }
        };
    }

    public synchronized TgFfiSqlPreparedStatement prepare(TgFfiContext context, String sql, List<TgFfiSqlPlaceholder> placeholders) {
        Objects.requireNonNull(sql, "sql must not be null");

        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var arg1 = allocateString(sql);
        MemorySegment arg2;
        int size;
        if (placeholders != null) {
            arg2 = allocateArray(placeholders);
            size = placeholders.size();
        } else {
            arg2 = MemorySegment.NULL;
            size = 0;
        }
        var out = allocateHandleOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_prepare(ctx, handle, arg1, arg2, size, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiSqlPreparedStatement(manager(), outHandle);
    }

    public synchronized TgFfiSqlPreparedStatement prepareFor(TgFfiContext context, String sql, List<TgFfiSqlPlaceholder> placeholders, Duration timeout) {
        Objects.requireNonNull(sql, "sql must not be null");

        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var arg1 = allocateString(sql);
        MemorySegment arg2;
        int size;
        if (placeholders != null) {
            arg2 = allocateArray(placeholders);
            size = placeholders.size();
        } else {
            arg2 = MemorySegment.NULL;
            size = 0;
        }
        var t = allocateDuration(timeout);
        var out = allocateHandleOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_prepare_for(ctx, handle, arg1, arg2, size, t, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiSqlPreparedStatement(manager(), outHandle);
    }

    public synchronized TgFfiJob<TgFfiSqlPreparedStatement> prepareAsync(TgFfiContext context, String sql, List<TgFfiSqlPlaceholder> placeholders) {
        Objects.requireNonNull(sql, "sql must not be null");

        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var arg1 = allocateString(sql);
        MemorySegment arg2;
        int size;
        if (placeholders != null) {
            arg2 = allocateArray(placeholders);
            size = placeholders.size();
        } else {
            arg2 = MemorySegment.NULL;
            size = 0;
        }
        var out = allocateHandleOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_prepare_async(ctx, handle, arg1, arg2, size, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiJob<>(manager(), outHandle) {
            @Override
            protected TgFfiSqlPreparedStatement valueToFfiObject(TgFfiObjectManager manager, MemorySegment valueHandle) {
                return new TgFfiSqlPreparedStatement(manager, valueHandle);
            }
        };
    }

    public synchronized TgFfiSqlExplainResult explain(TgFfiContext context, String sql) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var arg = allocateString(sql);
        var out = allocateHandleOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_explain(ctx, handle, arg, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiSqlExplainResult(manager(), outHandle);
    }

    public synchronized TgFfiSqlExplainResult explainFor(TgFfiContext context, String sql, Duration timeout) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var arg = allocateString(sql);
        var t = allocateDuration(timeout);
        var out = allocateHandleOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_explain_for(ctx, handle, arg, t, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiSqlExplainResult(manager(), outHandle);
    }

    public synchronized TgFfiJob<TgFfiSqlExplainResult> explainAsync(TgFfiContext context, String sql) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var arg = allocateString(sql);
        var out = allocateHandleOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_explain_async(ctx, handle, arg, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiJob<>(manager(), outHandle) {
            @Override
            protected TgFfiSqlExplainResult valueToFfiObject(TgFfiObjectManager manager, MemorySegment valueHandle) {
                return new TgFfiSqlExplainResult(manager, valueHandle);
            }
        };
    }

    public synchronized TgFfiSqlExplainResult preparedExplain(TgFfiContext context, TgFfiSqlPreparedStatement preparedStatement, List<TgFfiSqlParameter> parameters) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var ps = preparedStatement.handle();
        MemorySegment arg;
        int size;
        if (parameters != null) {
            arg = allocateArray(parameters);
            size = parameters.size();
        } else {
            arg = MemorySegment.NULL;
            size = 0;
        }
        var out = allocateHandleOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_prepared_explain(ctx, handle, ps, arg, size, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiSqlExplainResult(manager(), outHandle);
    }

    public synchronized TgFfiSqlExplainResult preparedExplainFor(TgFfiContext context, TgFfiSqlPreparedStatement preparedStatement, List<TgFfiSqlParameter> parameters, Duration timeout) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var ps = preparedStatement.handle();
        MemorySegment arg;
        int size;
        if (parameters != null) {
            arg = allocateArray(parameters);
            size = parameters.size();
        } else {
            arg = MemorySegment.NULL;
            size = 0;
        }
        var t = allocateDuration(timeout);
        var out = allocateHandleOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_prepared_explain_for(ctx, handle, ps, arg, size, t, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiSqlExplainResult(manager(), outHandle);
    }

    public synchronized TgFfiJob<TgFfiSqlExplainResult> preparedExplainAsync(TgFfiContext context, TgFfiSqlPreparedStatement preparedStatement, List<TgFfiSqlParameter> parameters) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var ps = preparedStatement.handle();
        MemorySegment arg;
        int size;
        if (parameters != null) {
            arg = allocateArray(parameters);
            size = parameters.size();
        } else {
            arg = MemorySegment.NULL;
            size = 0;
        }
        var out = allocateHandleOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_prepared_explain_async(ctx, handle, ps, arg, size, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiJob<>(manager(), outHandle) {
            @Override
            protected TgFfiSqlExplainResult valueToFfiObject(TgFfiObjectManager manager, MemorySegment valueHandle) {
                return new TgFfiSqlExplainResult(manager, valueHandle);
            }
        };
    }

    public synchronized TgFfiTransaction startTransaction(TgFfiContext context, TgFfiTransactionOption transactionOption) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var arg = transactionOption.handle();
        var out = allocateHandleOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_start_transaction(ctx, handle, arg, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiTransaction(manager(), outHandle);
    }

    public synchronized TgFfiTransaction startTransactionFor(TgFfiContext context, TgFfiTransactionOption transactionOption, Duration timeout) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var arg = transactionOption.handle();
        var t = allocateDuration(timeout);
        var out = allocateHandleOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_start_transaction_for(ctx, handle, arg, t, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiTransaction(manager(), outHandle);
    }

    public synchronized TgFfiJob<TgFfiTransaction> startTransactionAsync(TgFfiContext context, TgFfiTransactionOption transactionOption) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var arg = transactionOption.handle();
        var out = allocateHandleOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_start_transaction_async(ctx, handle, arg, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiJob<>(manager(), outHandle) {
            @Override
            protected TgFfiTransaction valueToFfiObject(TgFfiObjectManager manager, MemorySegment valueHandle) {
                return new TgFfiTransaction(manager, valueHandle);
            }
        };
    }

    public synchronized TgFfiTransactionStatus getTransactionStatus(TgFfiContext context, TgFfiTransaction transaction) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var arg = transaction.handle();
        var out = allocateHandleOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_get_transaction_status(ctx, handle, arg, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiTransactionStatus(manager(), outHandle);
    }

    public synchronized TgFfiTransactionStatus getTransactionStatusFor(TgFfiContext context, TgFfiTransaction transaction, Duration timeout) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var arg = transaction.handle();
        var t = allocateDuration(timeout);
        var out = allocateHandleOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_get_transaction_status_for(ctx, handle, arg, t, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiTransactionStatus(manager(), outHandle);
    }

    public synchronized TgFfiJob<TgFfiTransactionStatus> getTransactionStatusAsync(TgFfiContext context, TgFfiTransaction transaction) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var arg = transaction.handle();
        var out = allocateHandleOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_get_transaction_status_async(ctx, handle, arg, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiJob<>(manager(), outHandle) {
            @Override
            protected TgFfiTransactionStatus valueToFfiObject(TgFfiObjectManager manager, MemorySegment valueHandle) {
                return new TgFfiTransactionStatus(manager, valueHandle);
            }
        };
    }

    public synchronized TgFfiSqlExecuteResult execute(TgFfiContext context, TgFfiTransaction transaction, String sql) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var tx = transaction.handle();
        var arg = allocateString(sql);
        var out = allocateHandleOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_execute(ctx, handle, tx, arg, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiSqlExecuteResult(manager(), outHandle);
    }

    public synchronized TgFfiSqlExecuteResult executeFor(TgFfiContext context, TgFfiTransaction transaction, String sql, Duration timeout) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var tx = transaction.handle();
        var arg = allocateString(sql);
        var t = allocateDuration(timeout);
        var out = allocateHandleOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_execute_for(ctx, handle, tx, arg, t, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiSqlExecuteResult(manager(), outHandle);
    }

    public synchronized TgFfiJob<TgFfiSqlExecuteResult> executeAsync(TgFfiContext context, TgFfiTransaction transaction, String sql) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var tx = transaction.handle();
        var arg = allocateString(sql);
        var out = allocateHandleOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_execute_async(ctx, handle, tx, arg, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiJob<>(manager(), outHandle) {
            @Override
            protected TgFfiSqlExecuteResult valueToFfiObject(TgFfiObjectManager manager, MemorySegment valueHandle) {
                return new TgFfiSqlExecuteResult(manager, valueHandle);
            }
        };
    }

    public synchronized TgFfiSqlExecuteResult preparedExecute(TgFfiContext context, TgFfiTransaction transaction, TgFfiSqlPreparedStatement preparedStatement, List<TgFfiSqlParameter> parameters) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var tx = transaction.handle();
        var ps = preparedStatement.handle();
        MemorySegment arg;
        int size;
        if (parameters != null) {
            arg = allocateArray(parameters);
            size = parameters.size();
        } else {
            arg = MemorySegment.NULL;
            size = 0;
        }
        var out = allocateHandleOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_prepared_execute(ctx, handle, tx, ps, arg, size, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiSqlExecuteResult(manager(), outHandle);
    }

    public synchronized TgFfiSqlExecuteResult preparedExecuteFor(TgFfiContext context, TgFfiTransaction transaction, TgFfiSqlPreparedStatement preparedStatement, List<TgFfiSqlParameter> parameters,
            Duration timeout) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var tx = transaction.handle();
        var ps = preparedStatement.handle();
        MemorySegment arg;
        int size;
        if (parameters != null) {
            arg = allocateArray(parameters);
            size = parameters.size();
        } else {
            arg = MemorySegment.NULL;
            size = 0;
        }
        var t = allocateDuration(timeout);
        var out = allocateHandleOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_prepared_execute_for(ctx, handle, tx, ps, arg, size, t, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiSqlExecuteResult(manager(), outHandle);
    }

    public synchronized TgFfiJob<TgFfiSqlExecuteResult> preparedExecuteAsync(TgFfiContext context, TgFfiTransaction transaction, TgFfiSqlPreparedStatement preparedStatement,
            List<TgFfiSqlParameter> parameters) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var tx = transaction.handle();
        var ps = preparedStatement.handle();
        MemorySegment arg;
        int size;
        if (parameters != null) {
            arg = allocateArray(parameters);
            size = parameters.size();
        } else {
            arg = MemorySegment.NULL;
            size = 0;
        }
        var out = allocateHandleOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_prepared_execute_async(ctx, handle, tx, ps, arg, size, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiJob<>(manager(), outHandle) {
            @Override
            protected TgFfiSqlExecuteResult valueToFfiObject(TgFfiObjectManager manager, MemorySegment valueHandle) {
                return new TgFfiSqlExecuteResult(manager, valueHandle);
            }
        };
    }

    public synchronized TgFfiSqlQueryResult query(TgFfiContext context, TgFfiTransaction transaction, String sql) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var tx = transaction.handle();
        var arg = allocateString(sql);
        var out = allocateHandleOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_query(ctx, handle, tx, arg, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiSqlQueryResult(manager(), outHandle);
    }

    public synchronized TgFfiSqlQueryResult queryFor(TgFfiContext context, TgFfiTransaction transaction, String sql, Duration timeout) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var tx = transaction.handle();
        var arg = allocateString(sql);
        var t = allocateDuration(timeout);
        var out = allocateHandleOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_query_for(ctx, handle, tx, arg, t, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiSqlQueryResult(manager(), outHandle);
    }

    public synchronized TgFfiJob<TgFfiSqlQueryResult> queryAsync(TgFfiContext context, TgFfiTransaction transaction, String sql) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var tx = transaction.handle();
        var arg = allocateString(sql);
        var out = allocateHandleOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_query_async(ctx, handle, tx, arg, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiJob<>(manager(), outHandle) {
            @Override
            protected TgFfiSqlQueryResult valueToFfiObject(TgFfiObjectManager manager, MemorySegment valueHandle) {
                return new TgFfiSqlQueryResult(manager, valueHandle);
            }
        };
    }

    public synchronized TgFfiSqlQueryResult preparedQuery(TgFfiContext context, TgFfiTransaction transaction, TgFfiSqlPreparedStatement preparedStatement, List<TgFfiSqlParameter> parameters) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var tx = transaction.handle();
        var ps = preparedStatement.handle();
        MemorySegment arg;
        int size;
        if (parameters != null) {
            arg = allocateArray(parameters);
            size = parameters.size();
        } else {
            arg = MemorySegment.NULL;
            size = 0;
        }
        var out = allocateHandleOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_prepared_query(ctx, handle, tx, ps, arg, size, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiSqlQueryResult(manager(), outHandle);
    }

    public synchronized TgFfiSqlQueryResult preparedQueryFor(TgFfiContext context, TgFfiTransaction transaction, TgFfiSqlPreparedStatement preparedStatement, List<TgFfiSqlParameter> parameters,
            Duration timeout) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var tx = transaction.handle();
        var ps = preparedStatement.handle();
        MemorySegment arg;
        int size;
        if (parameters != null) {
            arg = allocateArray(parameters);
            size = parameters.size();
        } else {
            arg = MemorySegment.NULL;
            size = 0;
        }
        var t = allocateDuration(timeout);
        var out = allocateHandleOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_prepared_query_for(ctx, handle, tx, ps, arg, size, t, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiSqlQueryResult(manager(), outHandle);
    }

    public synchronized TgFfiJob<TgFfiSqlQueryResult> preparedQueryAsync(TgFfiContext context, TgFfiTransaction transaction, TgFfiSqlPreparedStatement preparedStatement,
            List<TgFfiSqlParameter> parameters) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var tx = transaction.handle();
        var ps = preparedStatement.handle();
        MemorySegment arg;
        int size;
        if (parameters != null) {
            arg = allocateArray(parameters);
            size = parameters.size();
        } else {
            arg = MemorySegment.NULL;
            size = 0;
        }
        var out = allocateHandleOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_prepared_query_async(ctx, handle, tx, ps, arg, size, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiJob<>(manager(), outHandle) {
            @Override
            protected TgFfiSqlQueryResult valueToFfiObject(TgFfiObjectManager manager, MemorySegment valueHandle) {
                return new TgFfiSqlQueryResult(manager, valueHandle);
            }
        };
    }

    public synchronized byte[] readBlob(TgFfiContext context, TgFfiTransaction transaction, TgFfiBlobReference blob) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var tx = transaction.handle();
        var arg1 = blob.handle();
        var out = allocatePtrOut();
        var sizeOut = allocateLongOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_read_blob(ctx, handle, tx, arg1, out, sizeOut);
        TgFfiRcUtil.throwIfError(rc, context);

        return outToBytesLong(out, sizeOut);
    }

    public synchronized byte[] readBlobFor(TgFfiContext context, TgFfiTransaction transaction, TgFfiBlobReference blob, Duration timeout) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var tx = transaction.handle();
        var arg1 = blob.handle();
        var t = allocateDuration(timeout);
        var out = allocatePtrOut();
        var sizeOut = allocateLongOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_read_blob_for(ctx, handle, tx, arg1, t, out, sizeOut);
        TgFfiRcUtil.throwIfError(rc, context);

        return outToBytesLong(out, sizeOut);
    }

    public synchronized String readClob(TgFfiContext context, TgFfiTransaction transaction, TgFfiClobReference clob) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var tx = transaction.handle();
        var arg1 = clob.handle();
        var out = allocatePtrOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_read_clob(ctx, handle, tx, arg1, out);
        TgFfiRcUtil.throwIfError(rc, context);

        return outToString(out);
    }

    public synchronized String readClobFor(TgFfiContext context, TgFfiTransaction transaction, TgFfiClobReference clob, Duration timeout) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var tx = transaction.handle();
        var arg1 = clob.handle();
        var t = allocateDuration(timeout);
        var out = allocatePtrOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_read_clob_for(ctx, handle, tx, arg1, t, out);
        TgFfiRcUtil.throwIfError(rc, context);

        return outToString(out);
    }

    public synchronized void copyBlobTo(TgFfiContext context, TgFfiTransaction transaction, TgFfiBlobReference blob, Path destination) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var tx = transaction.handle();
        var arg1 = blob.handle();
        var arg2 = allocateString(destination.toString());
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_copy_blob_to(ctx, handle, tx, arg1, arg2);
        TgFfiRcUtil.throwIfError(rc, context);
    }

    public synchronized void copyBlobToFor(TgFfiContext context, TgFfiTransaction transaction, TgFfiBlobReference blob, Path destination, Duration timeout) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var tx = transaction.handle();
        var arg1 = blob.handle();
        var arg2 = allocateString(destination.toString());
        var t = allocateDuration(timeout);
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_copy_blob_to_for(ctx, handle, tx, arg1, arg2, t);
        TgFfiRcUtil.throwIfError(rc, context);
    }

    public synchronized TgFfiVoidJob copyBlobToAsync(TgFfiContext context, TgFfiTransaction transaction, TgFfiBlobReference blob, Path destination) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var tx = transaction.handle();
        var arg1 = blob.handle();
        var arg2 = allocateString(destination.toString());
        var out = allocateHandleOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_copy_blob_to_async(ctx, handle, tx, arg1, arg2, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiVoidJob(manager(), outHandle);
    }

    public synchronized void copyClobTo(TgFfiContext context, TgFfiTransaction transaction, TgFfiClobReference clob, Path destination) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var tx = transaction.handle();
        var arg1 = clob.handle();
        var arg2 = allocateString(destination.toString());
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_copy_clob_to(ctx, handle, tx, arg1, arg2);
        TgFfiRcUtil.throwIfError(rc, context);
    }

    public synchronized void copyClobToFor(TgFfiContext context, TgFfiTransaction transaction, TgFfiClobReference clob, Path destination, Duration timeout) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var tx = transaction.handle();
        var arg1 = clob.handle();
        var arg2 = allocateString(destination.toString());
        var t = allocateDuration(timeout);
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_copy_clob_to_for(ctx, handle, tx, arg1, arg2, t);
        TgFfiRcUtil.throwIfError(rc, context);
    }

    public synchronized TgFfiVoidJob copyClobToAsync(TgFfiContext context, TgFfiTransaction transaction, TgFfiClobReference clob, Path destination) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var tx = transaction.handle();
        var arg1 = clob.handle();
        var arg2 = allocateString(destination.toString());
        var out = allocateHandleOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_copy_clob_to_async(ctx, handle, tx, arg1, arg2, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiVoidJob(manager(), outHandle);
    }

    public synchronized void commit(TgFfiContext context, TgFfiTransaction transaction, TgFfiCommitOption commitOption) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var tx = transaction.handle();
        var arg = commitOption.handle();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_commit(ctx, handle, tx, arg);
        TgFfiRcUtil.throwIfError(rc, context);
    }

    public synchronized void commitFor(TgFfiContext context, TgFfiTransaction transaction, TgFfiCommitOption commitOption, Duration timeout) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var tx = transaction.handle();
        var arg = commitOption.handle();
        var t = allocateDuration(timeout);
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_commit_for(ctx, handle, tx, arg, t);
        TgFfiRcUtil.throwIfError(rc, context);
    }

    public synchronized TgFfiVoidJob commitAsync(TgFfiContext context, TgFfiTransaction transaction, TgFfiCommitOption commitOption) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var tx = transaction.handle();
        var arg = commitOption.handle();
        var out = allocateHandleOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_commit_async(ctx, handle, tx, arg, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiVoidJob(manager(), outHandle);
    }

    public synchronized void rollback(TgFfiContext context, TgFfiTransaction transaction) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var tx = transaction.handle();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_rollback(ctx, handle, tx);
        TgFfiRcUtil.throwIfError(rc, context);
    }

    public synchronized void rollbackFor(TgFfiContext context, TgFfiTransaction transaction, Duration timeout) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var tx = transaction.handle();
        var t = allocateDuration(timeout);
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_rollback_for(ctx, handle, tx, t);
        TgFfiRcUtil.throwIfError(rc, context);
    }

    public synchronized TgFfiVoidJob rollbackAsync(TgFfiContext context, TgFfiTransaction transaction) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var tx = transaction.handle();
        var out = allocateHandleOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_rollback_async(ctx, handle, tx, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiVoidJob(manager(), outHandle);
    }

    @Override
    protected void dispose(MemorySegment handle) {
        tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_dispose(handle);
    }
}
