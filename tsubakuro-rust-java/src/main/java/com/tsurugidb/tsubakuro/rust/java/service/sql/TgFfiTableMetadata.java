package com.tsurugidb.tsubakuro.rust.java.service.sql;

import java.lang.foreign.MemorySegment;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.rc.TgFfiRcUtil;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObject;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObjectManager;

public class TgFfiTableMetadata extends TgFfiObject {

    private List<TgFfiSqlColumn> columns = null;
    private List<String> primaryKeys = null;

    TgFfiTableMetadata(TgFfiObjectManager manager, MemorySegment handle) {
        super(manager, handle);
    }

    public synchronized String getDatabaseName(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var out = allocatePtrOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_table_metadata_get_database_name(ctx, handle, out);
        TgFfiRcUtil.throwIfError(rc, context);

        return outToString(out);
    }

    public synchronized String getSchemaName(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var out = allocatePtrOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_table_metadata_get_schema_name(ctx, handle, out);
        TgFfiRcUtil.throwIfError(rc, context);

        return outToString(out);
    }

    public synchronized String getTableName(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var out = allocatePtrOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_table_metadata_get_table_name(ctx, handle, out);
        TgFfiRcUtil.throwIfError(rc, context);

        return outToString(out);
    }

    public synchronized String getDescription(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var out = allocatePtrOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_table_metadata_get_description(ctx, handle, out);
        TgFfiRcUtil.throwIfError(rc, context);

        return outToString(out);
    }

    public synchronized List<TgFfiSqlColumn> getColumns(TgFfiContext context) {
        if (this.columns == null) {
            this.columns = createColumns(context);
        }
        return this.columns;
    }

    private List<TgFfiSqlColumn> createColumns(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();

        int size;
        {
            var out = allocateIntOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_table_metadata_get_columns_size(ctx, handle, out);
            TgFfiRcUtil.throwIfError(rc, context);

            size = outToInt(out);
        }

        var list = new ArrayList<TgFfiSqlColumn>(size);
        for (int i = 0; i < size; i++) {
            var out = allocateHandleOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_table_metadata_get_columns_value(ctx, handle, i, out);
            TgFfiRcUtil.throwIfError(rc, context);

            var outHandle = outToHandle(out);
            list.add(new TgFfiSqlColumn(manager(), outHandle));
        }

        return Collections.unmodifiableList(list);
    }

    public synchronized List<String> getPrimaryKeys(TgFfiContext context) {
        if (this.primaryKeys == null) {
            this.primaryKeys = createPrimaryKeys(context);
        }
        return this.primaryKeys;
    }

    private List<String> createPrimaryKeys(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();

        var out = allocatePtrOut();
        var sizeOut = allocateIntOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_table_metadata_get_primary_keys(ctx, handle, out, sizeOut);
        TgFfiRcUtil.throwIfError(rc, context);

        return outToStringList(out, sizeOut);
    }

    @Override
    public void close() {
        List<RuntimeException> list = null;
        if (this.columns != null) {
            for (var column : columns) {
                try {
                    column.close();
                } catch (RuntimeException e) {
                    if (list == null) {
                        list = new ArrayList<>();
                    }
                    list.add(e);
                }
            }
        }

        try {
            super.close();
        } catch (RuntimeException e) {
            if (list != null) {
                for (var re : list) {
                    e.addSuppressed(re);
                }
            }
            throw e;
        }

        if (list != null) {
            RuntimeException e = null;
            for (var re : list) {
                if (e == null) {
                    e = re;
                } else {
                    e.addSuppressed(re);
                }
            }
            throw e;
        }
    }

    @Override
    protected void dispose(MemorySegment handle) {
        tsubakuro_rust_ffi_h.tsurugi_ffi_table_metadata_dispose(handle);
    }
}
