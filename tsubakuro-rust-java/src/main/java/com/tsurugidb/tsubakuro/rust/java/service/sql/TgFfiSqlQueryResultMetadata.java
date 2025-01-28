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

public class TgFfiSqlQueryResultMetadata extends TgFfiObject {

	private List<TgFfiSqlColumn> columns = null;

	TgFfiSqlQueryResultMetadata(TgFfiObjectManager manager, MemorySegment handle) {
		super(manager, handle);
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
			var out = allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_metadata_get_columns_size(ctx, handle, out);
			TgFfiRcUtil.throwIfError(rc, context);

			size = outToInt(out);
		}

		var list = new ArrayList<TgFfiSqlColumn>(size);
		for (int i = 0; i < size; i++) {
			var out = allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_metadata_get_columns_value(ctx, handle, i, out);
			TgFfiRcUtil.throwIfError(rc, context);

			var outHandle = outToHandle(out);
			list.add(new TgFfiSqlColumn(manager(), outHandle));
		}

		return Collections.unmodifiableList(list);
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
		tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_metadata_dispose(handle);
	}
}
