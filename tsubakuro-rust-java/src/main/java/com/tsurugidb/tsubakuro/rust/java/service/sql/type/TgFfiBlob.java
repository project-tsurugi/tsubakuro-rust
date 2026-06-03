package com.tsurugidb.tsubakuro.rust.java.service.sql.type;

import java.lang.foreign.MemorySegment;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObject;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObjectManager;

public class TgFfiBlob extends TgFfiObject {

    public TgFfiBlob(TgFfiObjectManager manager, MemorySegment handle) {
        super(manager, handle);
    }

    @Override
    protected void dispose(MemorySegment handle) {
        tsubakuro_rust_ffi_h.tsurugi_ffi_blob_dispose(handle);
    }
}
