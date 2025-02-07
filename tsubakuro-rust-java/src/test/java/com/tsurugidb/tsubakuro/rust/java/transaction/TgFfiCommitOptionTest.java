package com.tsurugidb.tsubakuro.rust.java.transaction;

import static org.junit.jupiter.api.Assertions.assertEquals;

import java.lang.foreign.MemorySegment;
import java.time.Duration;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.service.sql.TgFfiSqlClient;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiTester;

class TgFfiCommitOptionTest extends TgFfiTester {

    @Test
    void create() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager); //
                var target = TgFfiCommitOption.create(context)) {
        }

        try (var target = TgFfiCommitOption.create(manager)) {
        }
    }

    @Test
    void create_argError() {
        var manager = getFfiObjectManager();

        try (var contextObject = TgFfiContext.create(manager)) {
            var context = contextObject.handle();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_commit_option_create(context, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
    }

    @Test
    void set_commit_type() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager); //
                var target = TgFfiCommitOption.create(context)) {
            assertEquals(TgFfiCommitType.UNSPECIFIED, target.getCommitType(context));

            target.setCommitType(context, TgFfiCommitType.AVAILABLE);

            var type = target.getCommitType(context);
            assertEquals(TgFfiCommitType.AVAILABLE, type);
        }
    }

    @Test
    void set_commit_type_argError() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var arg = TgFfiCommitType.AVAILABLE.value();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_commit_option_set_commit_type(ctx, handle, arg);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
    }

    @Test
    void get_commit_type_argError() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var out = manager.allocateIntOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_commit_option_get_commit_type(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager); //
                var target = TgFfiCommitOption.create(context)) {
            var ctx = context.handle();
            var handle = target.handle();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_commit_option_get_commit_type(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }

    @ParameterizedTest
    @ValueSource(strings = { DIRECT, DIRECT_FOR, TAKE, TAKE_FOR, TAKE_IF_READY })
    void set_auto_dispose(String pattern) {
        setAutoDispose(false, pattern);
        setAutoDispose(true, pattern);
    }

    private void setAutoDispose(boolean autoDispose, String pattern) {
        var manager = getFfiObjectManager();
        try (var client = createSqlClient(); //
                var context = TgFfiContext.create(manager); //
                var transaction = createTransaction(client, pattern)) {
            doCommit(client, transaction, autoDispose, pattern);
            doClose(transaction, pattern);
        }
    }

    @Test
    void set_auto_dispose_argError() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var arg = true;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_commit_option_set_auto_dispose(ctx, handle, arg);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
    }

    @Test
    void get_auto_dispose_argError() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var out = manager.allocateBooleanOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_commit_option_get_auto_dispose(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager); //
                var target = TgFfiCommitOption.create(context)) {
            var ctx = context.handle();
            var handle = target.handle();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_commit_option_get_auto_dispose(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }

    private TgFfiTransaction createTransaction(TgFfiSqlClient client, String pattern) {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager); //
                var transactionOption = TgFfiTransactionOption.create(context)) {
            switch (pattern) {
            case DIRECT:
                return client.startTransaction(context, transactionOption);
            case DIRECT_FOR:
                return client.startTransactionFor(context, transactionOption, Duration.ofSeconds(5));
            default:
                try (var job = client.startTransactionAsync(context, transactionOption)) {
                    return jobTake(job, pattern);
                }
            }
        }
    }

    private void doCommit(TgFfiSqlClient client, TgFfiTransaction transaction, boolean autoDispose, String pattern) {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager); //
                var commitOption = TgFfiCommitOption.create(context)) {
            commitOption.setAutoDispose(context, autoDispose);

            assertEquals(autoDispose, commitOption.getAutoDispose(context));

            switch (pattern) {
            case DIRECT:
                client.commit(context, transaction, commitOption);
                break;
            case DIRECT_FOR:
                client.commitFor(context, transaction, commitOption, Duration.ofSeconds(5));
                break;
            default:
                try (var job = client.commitAsync(context, transaction, commitOption)) {
                    jobTake(job, pattern);
                }
                break;
            }
        }
    }

    private void doClose(TgFfiTransaction transaction, String pattern) {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            switch (pattern) {
            case DIRECT:
            default:
                transaction.close(context);
                break;
            case DIRECT_FOR:
                transaction.closeFor(context, Duration.ofSeconds(5));
                break;
            }
        }
    }
}
