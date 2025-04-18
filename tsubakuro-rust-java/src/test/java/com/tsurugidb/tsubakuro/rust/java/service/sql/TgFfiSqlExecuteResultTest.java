package com.tsurugidb.tsubakuro.rust.java.service.sql;

import static org.junit.jupiter.api.Assertions.assertEquals;

import java.lang.foreign.MemorySegment;
import java.time.Duration;
import java.util.List;
import java.util.Map;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.session.TgFfiConnectionOption;
import com.tsurugidb.tsubakuro.rust.java.session.TgFfiSession;
import com.tsurugidb.tsubakuro.rust.java.transaction.TgFfiCommitOption;
import com.tsurugidb.tsubakuro.rust.java.transaction.TgFfiTransactionOption;
import com.tsurugidb.tsubakuro.rust.java.transaction.TgFfiTransactionType;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiTester;

class TgFfiSqlExecuteResultTest extends TgFfiTester {

    @BeforeEach
    void before() {
        dropIfExists("test");

        try (var context = TgFfiContext.create(getFfiObjectManager())) {
            var sql = """
                    create table test (
                      foo int primary key,
                      bar bigint,
                      zzz varchar(10)
                    )""";
            try (var er = getExecuteResult(false, DIRECT, sql)) {
                assertEquals(Map.of(), er.getCounters(context));
                assertEquals(0, er.getInsertedRows(context));
                assertEquals(0, er.getUpdatedRows(context));
                assertEquals(0, er.getMergedRows(context));
                assertEquals(0, er.getDeletedRows(context));
                assertEquals(0, er.getRows(context));
            }
        }

        executeSql("insert into test values(1, 11, 'aa')");
        executeSql("insert into test values(2, 22, 'bb')");
        executeSql("insert into test values(3, 33, 'cc')");
    }

    @ParameterizedTest
    @ValueSource(strings = { DIRECT, DIRECT_FOR, TAKE, TAKE_FOR, TAKE_IF_READY })
    void get_rows(String pattern) {
        getRows(false, pattern);
    }

    @ParameterizedTest
    @ValueSource(strings = { DIRECT, DIRECT_FOR, TAKE, TAKE_FOR, TAKE_IF_READY })
    void get_rows_fromPs(String pattern) {
        getRows(true, pattern);
    }

    private void getRows(boolean prepare, String pattern) {
        var manager = getFfiObjectManager();
        var context = TgFfiContext.create(manager);

        try (var executeResult = getExecuteResult(prepare, pattern, "insert into test values(4, 44, 'dd')")) {
            assertEquals(Map.of(TgFfiSqlCounterType.INSERTED_ROWS, 1L), executeResult.getCounters(context));
            assertEquals(1, executeResult.getInsertedRows(context));
            assertEquals(0, executeResult.getUpdatedRows(context));
            assertEquals(0, executeResult.getMergedRows(context));
            assertEquals(0, executeResult.getDeletedRows(context));
            assertEquals(1, executeResult.getRows(context));
        }
        try (var executeResult = getExecuteResult(prepare, pattern, "update test set bar = 99")) {
            assertEquals(Map.of(TgFfiSqlCounterType.UPDATED_ROWS, 4L), executeResult.getCounters(context));
            assertEquals(0, executeResult.getInsertedRows(context));
            assertEquals(4, executeResult.getUpdatedRows(context));
            assertEquals(0, executeResult.getMergedRows(context));
            assertEquals(0, executeResult.getDeletedRows(context));
            assertEquals(4, executeResult.getRows(context));
        }
        try (var executeResult = getExecuteResult(prepare, pattern, "insert or replace into test values(2, 222, 'bbb')")) {
            assertEquals(Map.of(TgFfiSqlCounterType.MERGED_ROWS, 1L), executeResult.getCounters(context));
            assertEquals(0, executeResult.getInsertedRows(context));
            assertEquals(0, executeResult.getUpdatedRows(context));
            assertEquals(1, executeResult.getMergedRows(context));
            assertEquals(0, executeResult.getDeletedRows(context));
            assertEquals(1, executeResult.getRows(context));
        }
        try (var executeResult = getExecuteResult(prepare, pattern, "delete from test where foo = 3")) {
            assertEquals(Map.of(TgFfiSqlCounterType.DELETED_ROWS, 1L), executeResult.getCounters(context));
            assertEquals(0, executeResult.getInsertedRows(context));
            assertEquals(0, executeResult.getUpdatedRows(context));
            assertEquals(0, executeResult.getMergedRows(context));
            assertEquals(1, executeResult.getDeletedRows(context));
            assertEquals(1, executeResult.getRows(context));
        }
        try (var executeResult = getExecuteResult(prepare, pattern, "delete from test where foo = 3")) {
            assertEquals(Map.of(TgFfiSqlCounterType.DELETED_ROWS, 0L), executeResult.getCounters(context));
            assertEquals(0, executeResult.getInsertedRows(context));
            assertEquals(0, executeResult.getUpdatedRows(context));
            assertEquals(0, executeResult.getMergedRows(context));
            assertEquals(0, executeResult.getDeletedRows(context));
            assertEquals(0, executeResult.getRows(context));
        }
    }

    @Test
    void get_rows_argError() {
        var manager = getFfiObjectManager();
        var context = TgFfiContext.create(manager);

        {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var out = manager.allocateLongOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_execute_result_get_rows(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var executeResult = getExecuteResult(false, DIRECT, "insert or replace into test values(2, 222, 'bbb')")) {
            var ctx = context.handle();
            var handle = executeResult.handle();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_execute_result_get_rows(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }

    protected TgFfiSqlExecuteResult getExecuteResult(boolean prepare, String pattern, String sql) {
        var manager = getFfiObjectManager();
        try (var context = TgFfiContext.create(manager); //
                var connectionOption = TgFfiConnectionOption.create(context)) {
            connectionOption.setEndpointUrl(context, getEndpoint());
            connectionOption.setApplicationName(context, "tsubakuro-rust-java/test");
            connectionOption.setSessionLabel(context, "tsubakuro-rust-java/test.session");

            try (var session = TgFfiSession.connect(context, connectionOption); //
                    var client = session.makeSqlClient(context); //
                    var ps = client.prepare(context, sql, List.of()); //
                    var transactionOption = TgFfiTransactionOption.create(context)) {
                transactionOption.setTransactionType(context, TgFfiTransactionType.SHORT);
                transactionOption.setTransactionLabel(context, "tsubakuro-rust-java/execute()");

                try (var transaction = client.startTransaction(context, transactionOption)) {
                    TgFfiSqlExecuteResult executeResult;
                    if (prepare) {
                        switch (pattern) {
                        case DIRECT:
                            executeResult = client.preparedExecute(context, transaction, ps, List.of());
                            break;
                        case DIRECT_FOR:
                            executeResult = client.preparedExecuteFor(context, transaction, ps, List.of(), Duration.ofSeconds(5));
                            break;
                        default:
                            try (var job = client.preparedExecuteAsync(context, transaction, ps, List.of())) {
                                executeResult = jobTake(job, pattern);
                            }
                            break;
                        }
                    } else {
                        switch (pattern) {
                        case DIRECT:
                            executeResult = client.execute(context, transaction, sql);
                            break;
                        case DIRECT_FOR:
                            executeResult = client.executeFor(context, transaction, sql, Duration.ofSeconds(5));
                            break;
                        default:
                            try (var job = client.executeAsync(context, transaction, sql)) {
                                executeResult = jobTake(job, pattern);
                            }
                            break;
                        }
                    }

                    try (var commitOption = TgFfiCommitOption.create(context)) {
                        client.commit(context, transaction, commitOption);
                    }

                    return executeResult;
                }
            }
        }
    }
}
