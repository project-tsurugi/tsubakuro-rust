package com.tsurugidb.tsubakuro.rust.java.service.lob;

import java.lang.foreign.MemorySegment;
import java.time.Duration;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.rc.TgFfiRcUtil;
import com.tsurugidb.tsubakuro.rust.java.service.sql.type.TgFfiBlob;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObject;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObjectManager;

public class TgFfiBlobUploader extends TgFfiObject {

    public TgFfiBlobUploader(TgFfiObjectManager manager, MemorySegment handle) {
        super(manager, handle);
    }

    public synchronized void uploadChunk(TgFfiContext context, byte[] value, Duration timeout) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var arg1 = allocateBytes(value);
        long size = value.length;
        var t = allocateDuration(timeout);
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_blob_uploader_upload_chunk(ctx, handle, arg1, size, t);
        TgFfiRcUtil.throwIfError(rc, context);
    }

    public synchronized TgFfiBlob finish(TgFfiContext context, Duration timeout) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var t = allocateDuration(timeout);
        var out = allocatePtrOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_blob_uploader_finish(ctx, handle, t, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiBlob(manager(), outHandle);
    }

    public synchronized void cancel(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_blob_uploader_cancel(ctx, handle);
        TgFfiRcUtil.throwIfError(rc, context);
    }

    @Override
    protected void dispose(MemorySegment handle) {
        tsubakuro_rust_ffi_h.tsurugi_ffi_blob_uploader_dispose(handle);
    }
}
