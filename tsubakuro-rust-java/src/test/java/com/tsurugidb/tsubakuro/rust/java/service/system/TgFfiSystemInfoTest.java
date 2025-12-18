package com.tsurugidb.tsubakuro.rust.java.service.system;

import static org.junit.jupiter.api.Assertions.assertEquals;

import java.lang.foreign.MemorySegment;
import java.time.Duration;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;

import com.tsurugidb.iceaxe.TsurugiConnector;
import com.tsurugidb.iceaxe.system.TsurugiSystemInfo;
import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.session.TgFfiConnectionOption;
import com.tsurugidb.tsubakuro.rust.java.session.TgFfiSession;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiTester;

class TgFfiSystemInfoTest extends TgFfiTester {

    @ParameterizedTest
    @ValueSource(strings = { DIRECT, DIRECT_FOR, TAKE, TAKE_FOR, TAKE_IF_READY })
    void systemInfo(String pattern) {
        var manager = getFfiObjectManager();

        try (var systemInfo = getSystemInfo(pattern); //
                var context = TgFfiContext.create(manager)) {
            var name = systemInfo.getName(context);
            var version = systemInfo.getVersion(context);

            var expected = expectedSystemInfo();
            assertEquals(expected.getName(), name);
            assertEquals(expected.getVersion(), version);
        }
    }

    private TsurugiSystemInfo expectedSystemInfo() {
        var connector = TsurugiConnector.of(getEndpointJava(), getCredentialJava());
        try (var session = connector.createSession()) {
            return session.getSystemInfo();
        } catch (RuntimeException e) {
            throw e;
        } catch (Exception e) {
            throw new RuntimeException(e);
        }
    }

    @Test
    void argError() {
        var manager = getFfiObjectManager();
        try (var metadata = getSystemInfo(DIRECT); //
                var context = TgFfiContext.create(manager)) {
            get_name_argError(context, metadata);
            get_version_argError(context, metadata);
        }
    }

    private void get_name_argError(TgFfiContext context, TgFfiSystemInfo metadata) {
        var manager = getFfiObjectManager();

        {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var out = manager.allocatePtrOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_system_info_get_name(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        {
            var ctx = context.handle();
            var handle = metadata.handle();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_system_info_get_name(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }

    private void get_version_argError(TgFfiContext context, TgFfiSystemInfo metadata) {
        var manager = getFfiObjectManager();

        {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var out = manager.allocatePtrOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_system_info_get_version(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        {
            var ctx = context.handle();
            var handle = metadata.handle();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_system_info_get_version(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }

    private TgFfiSystemInfo getSystemInfo(String pattern) {
        var manager = getFfiObjectManager();

        var context = TgFfiContext.create(manager);

        var connectionOption = TgFfiConnectionOption.create(context);
        connectionOption.setEndpointUrl(context, getEndpoint());
        connectionOption.setCredential(context, getCredential(context));

        try (var session = TgFfiSession.connect(context, connectionOption); //
                var client = session.makeSystemClient(context)) {
            switch (pattern) {
            case DIRECT:
                return client.getSystemInfo(context);
            case DIRECT_FOR:
                return client.getSystemInfoFor(context, Duration.ofSeconds(5));
            default:
                try (var job = client.getSystemInfoAsync(context)) {
                    return jobTake(job, pattern);
                }
            }
        }
    }
}
