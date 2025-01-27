package com.tsurugidb.tsubakuro.rust.java.service.sql;

import java.lang.foreign.MemorySegment;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.rc.TgFfiRcUtil;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObject;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObjectManager;

public class TgFfiSqlClient extends TgFfiObject {

	public TgFfiSqlClient(TgFfiObjectManager manager, MemorySegment handle) {
		super(manager, handle);
	}

	public synchronized TgFfiTableList listTables(TgFfiContext context) {
		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var out = allocatePtr();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_list_tables(ctx, handle, out);
		TgFfiRcUtil.throwIfError(rc, context);

		var tableList = outToHandle(out);
		return new TgFfiTableList(manager(), tableList);
	}

	public synchronized TgFfiTableMetadata getTableMetadata(TgFfiContext context, String tableName) {
		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var arg = allocateString(tableName);
		var out = allocatePtr();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_get_table_metadata(ctx, handle, arg, out);
		TgFfiRcUtil.throwIfError(rc, context);

		var tableMetadata = outToHandle(out);
		return new TgFfiTableMetadata(manager(), tableMetadata);
	}

	@Override
	protected void dispose(MemorySegment handle) {
		tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_dispose(handle);
	}
}
