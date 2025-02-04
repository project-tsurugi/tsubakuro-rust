package com.tsurugidb.tsubakuro.rust.java.context;

import java.lang.foreign.MemorySegment;
import java.util.Objects;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.rc.TgFfiRcType;
import com.tsurugidb.tsubakuro.rust.java.rc.TgFfiRcUtil;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObject;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObjectManager;

public class TgFfiContext extends TgFfiObject {

    public static TgFfiContext create(TgFfiObjectManager manager) {
        Objects.requireNonNull(manager, "manager must not be null");

        var out = manager.allocatePtr();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_context_create(out);
        TgFfiRcUtil.throwIfError(rc);

        var outHandle = outToHandle(out);
        return new TgFfiContext(manager, outHandle);
    }

    TgFfiContext(TgFfiObjectManager manager, MemorySegment handle) {
        super(manager, handle);
    }

    public synchronized int getReturnCode() {
        var handle = handle();
        var out = allocatePtr();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_context_get_return_code(handle, out);
        TgFfiRcUtil.throwIfError(rc);

        return outToInt(out);
    }

    public synchronized String getErrorName() {
        var handle = handle();
        var out = allocatePtr();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_context_get_error_name(handle, out);
        TgFfiRcUtil.throwIfError(rc);

        return outToString(out);
    }

    public synchronized TgFfiRcType getErrorType() {
        var handle = handle();
        var out = allocatePtr();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_context_get_error_type(handle, out);
        TgFfiRcUtil.throwIfError(rc);

        int outInt = outToInt(out);
        return TgFfiRcType.of(outInt);
    }

    public synchronized String getErrorMessage() {
        var handle = handle();
        var out = allocatePtr();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_context_get_error_message(handle, out);
        TgFfiRcUtil.throwIfError(rc);

        return outToString(out);
    }

    public synchronized int getServerErrorCategoryNumber() {
        var handle = handle();
        var out = allocatePtr();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_context_get_server_error_category_number(handle, out);
        TgFfiRcUtil.throwIfError(rc);

        return outToInt(out);
    }

    public synchronized String getServerErrorCategoryStr() {
        var handle = handle();
        var out = allocatePtr();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_context_get_server_error_category_str(handle, out);
        TgFfiRcUtil.throwIfError(rc);

        return outToString(out);
    }

    public synchronized int getServerErrorCodeNumber() {
        var handle = handle();
        var out = allocatePtr();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_context_get_server_error_code_number(handle, out);
        TgFfiRcUtil.throwIfError(rc);

        return outToInt(out);
    }

    public synchronized String getServerErrorStructuredCode() {
        var handle = handle();
        var out = allocatePtr();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_context_get_server_error_structured_code(handle, out);
        TgFfiRcUtil.throwIfError(rc);

        return outToString(out);
    }

    @Override
    protected void dispose(MemorySegment handle) {
        tsubakuro_rust_ffi_h.tsurugi_ffi_context_dispose(handle);
    }
}
