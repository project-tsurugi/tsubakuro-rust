package com.tsurugidb.tsubakuro.rust.java.service.lob;

import java.lang.foreign.MemorySegment;
import java.time.Duration;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.rc.TgFfiRcUtil;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObject;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObjectManager;

public class TgFfiBlobDownloader extends TgFfiObject {

    public TgFfiBlobDownloader(TgFfiObjectManager manager, MemorySegment handle) {
        super(manager, handle);
    }

    public synchronized byte[] downloadChunk(TgFfiContext context, long length, Duration timeout) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var arg1 = length;
        var t = allocateDuration(timeout);
        var out = allocatePtrOut();
        var sizeOut = allocateLongOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_blob_downloader_download_chunk(ctx, handle, arg1, t, out, sizeOut);
        TgFfiRcUtil.throwIfError(rc, context);

        return outToBytesLong(out, sizeOut);
    }

    public synchronized long downloadChunkInto(TgFfiContext context, byte[] buffer, Duration timeout) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var arg1 = allocateBytes(buffer);
        var size = buffer.length;
        var t = allocateDuration(timeout);
        var out = allocateLongOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_blob_downloader_download_chunk_into(ctx, handle, arg1, size, t, out);
        TgFfiRcUtil.throwIfError(rc, context);

        long length = outToLong(out);
        MemorySegment.copy(arg1, 0, MemorySegment.ofArray(buffer), 0, length);
        return length;
    }

    @Override
    protected void dispose(MemorySegment handle) {
        tsubakuro_rust_ffi_h.tsurugi_ffi_blob_downloader_dispose(handle);
    }
}
