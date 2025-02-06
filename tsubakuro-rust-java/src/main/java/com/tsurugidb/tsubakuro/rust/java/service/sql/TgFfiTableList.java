package com.tsurugidb.tsubakuro.rust.java.service.sql;

import java.lang.foreign.MemorySegment;
import java.util.List;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.rc.TgFfiRcUtil;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObject;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObjectManager;

public class TgFfiTableList extends TgFfiObject {

    private List<String> tableNames = null;

    TgFfiTableList(TgFfiObjectManager manager, MemorySegment handle) {
        super(manager, handle);
    }

    public synchronized List<String> getTableNames(TgFfiContext context) {
        if (this.tableNames == null) {
            this.tableNames = createTablesNames(context);
        }
        return this.tableNames;
    }

    private List<String> createTablesNames(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();

        var out = allocatePtrOut();
        var sizeOut = allocateIntOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_table_list_get_table_names(ctx, handle, out, sizeOut);
        TgFfiRcUtil.throwIfError(rc, context);

        return outToStringList(out, sizeOut);
    }

    @Override
    protected void dispose(MemorySegment handle) {
        tsubakuro_rust_ffi_h.tsurugi_ffi_table_list_dispose(handle);
    }
}
