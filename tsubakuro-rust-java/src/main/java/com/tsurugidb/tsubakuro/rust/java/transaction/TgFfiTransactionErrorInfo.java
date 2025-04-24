package com.tsurugidb.tsubakuro.rust.java.transaction;

import java.lang.foreign.MemorySegment;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.rc.TgFfiRcUtil;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObject;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObjectManager;

public class TgFfiTransactionErrorInfo extends TgFfiObject {

    public TgFfiTransactionErrorInfo(TgFfiObjectManager manager, MemorySegment handle) {
        super(manager, handle);
    }

    public synchronized boolean isNormal(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var out = allocateBooleanOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_error_info_is_normal(ctx, handle, out);
        TgFfiRcUtil.throwIfError(rc, context);

        return outToBoolean(out);
    }

    public synchronized boolean isError(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var out = allocateBooleanOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_error_info_is_error(ctx, handle, out);
        TgFfiRcUtil.throwIfError(rc, context);

        return outToBoolean(out);
    }

    public synchronized String getServerErrorName(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var out = allocatePtrOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_error_info_get_server_error_name(ctx, handle, out);
        TgFfiRcUtil.throwIfError(rc, context);

        return outToString(out);
    }

    public synchronized String getServerErrorMessage(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var out = allocatePtrOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_error_info_get_server_error_message(ctx, handle, out);
        TgFfiRcUtil.throwIfError(rc, context);

        return outToString(out);
    }

    public synchronized int getServerErrorCategoryNumber(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var out = allocateIntOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_error_info_get_server_error_category_number(ctx, handle, out);
        TgFfiRcUtil.throwIfError(rc, context);

        return outToInt(out);
    }

    public synchronized String getServerErrorCategoryStr(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var out = allocatePtrOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_error_info_get_server_error_category_str(ctx, handle, out);
        TgFfiRcUtil.throwIfError(rc, context);

        return outToString(out);
    }

    public synchronized int getServerErrorCodeNumber(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var out = allocateIntOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_error_info_get_server_error_code_number(ctx, handle, out);
        TgFfiRcUtil.throwIfError(rc, context);

        return outToInt(out);
    }

    public synchronized String getServerErrorStructuredCode(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var out = allocatePtrOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_error_info_get_server_error_structured_code(ctx, handle, out);
        TgFfiRcUtil.throwIfError(rc, context);

        return outToString(out);
    }

    @Override
    protected void dispose(MemorySegment handle) {
        tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_error_info_dispose(handle);
    }
}
