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

		int size;
		{
			var out = allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_table_list_get_table_names_size(ctx, handle, out);
			TgFfiRcUtil.throwIfError(rc, context);

			size = outToInt(out);
		}

		var list = new ArrayList<String>(size);
		for (int i = 0; i < size; i++) {
			var out = allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_table_list_get_table_names_element(ctx, handle, i, out);
			TgFfiRcUtil.throwIfError(rc, context);

			String tableName = outToString(out);
			list.add(tableName);
		}

		return Collections.unmodifiableList(list);
	}

	@Override
	protected void dispose(MemorySegment handle) {
		tsubakuro_rust_ffi_h.tsurugi_ffi_table_list_dispose(handle);
	}
}
