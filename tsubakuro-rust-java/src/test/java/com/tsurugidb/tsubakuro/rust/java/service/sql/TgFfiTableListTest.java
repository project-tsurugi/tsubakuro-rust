package com.tsurugidb.tsubakuro.rust.java.service.sql;

import static org.junit.jupiter.api.Assertions.*;

import java.lang.foreign.MemorySegment;
import java.time.Duration;
import java.util.List;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.session.TgFfiConnectionOption;
import com.tsurugidb.tsubakuro.rust.java.session.TgFfiSession;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiTester;

class TgFfiTableListTest extends TgFfiTester {

    @ParameterizedTest
    @ValueSource(strings = { DIRECT, DIRECT_FOR, TAKE, TAKE_FOR, TAKE_IF_READY })
    void get_table_names(String pattern) {
        var manager = getFfiObjectManager();
        var context = TgFfiContext.create(manager);

        dropIfExists("test");

        try (var tableList = getTableList(pattern)) {
            List<String> tableNames = tableList.getTableNames(context);
            assertFalse(tableNames.contains("test"));
        }

        executeSql("create table test (pk int primary key)");

        try (var tableList = getTableList(pattern)) {
            List<String> tableNames = tableList.getTableNames(context);
            assertTrue(tableNames.contains("test"));
        }
    }

    @Test
    void get_table_names_argError() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager); //
                var tableList = getTableList(DIRECT)) {
            {
                var ctx = context.handle();
                var handle = MemorySegment.NULL;
                var out = manager.allocatePtrOut();
                var sout = manager.allocateIntOut();
                var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_table_list_get_table_names(ctx, handle, out, sout);
                assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
            }
            {
                var ctx = context.handle();
                var handle = tableList.handle();
                var out = MemorySegment.NULL;
                var sout = manager.allocateIntOut();
                var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_table_list_get_table_names(ctx, handle, out, sout);
                assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
            }
            {
                var ctx = context.handle();
                var handle = tableList.handle();
                var out = manager.allocatePtrOut();
                var sout = MemorySegment.NULL;
                var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_table_list_get_table_names(ctx, handle, out, sout);
                assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
            }
        }
    }

    private TgFfiTableList getTableList(String pattern) {
        var manager = getFfiObjectManager();

        var context = TgFfiContext.create(manager);

        var connectionOption = TgFfiConnectionOption.create(context);
        connectionOption.setEndpointUrl(context, getEndpoint());
        connectionOption.setCredential(context, getCredential(context));

        try (var session = TgFfiSession.connect(context, connectionOption); //
                var client = session.makeSqlClient(context)) {
            switch (pattern) {
            case DIRECT:
                return client.listTables(context);
            case DIRECT_FOR:
                return client.listTablesFor(context, Duration.ofSeconds(5));
            default:
                try (var job = client.listTablesAsync(context)) {
                    return jobTake(job, pattern);
                }
            }
        }
    }
}
