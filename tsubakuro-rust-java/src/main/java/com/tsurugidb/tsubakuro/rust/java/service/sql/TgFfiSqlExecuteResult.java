package com.tsurugidb.tsubakuro.rust.java.service.sql;

import java.lang.foreign.MemorySegment;
import java.lang.foreign.ValueLayout;
import java.util.EnumMap;
import java.util.Map;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.rc.TgFfiRcUtil;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObject;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObjectManager;

public class TgFfiSqlExecuteResult extends TgFfiObject {

	TgFfiSqlExecuteResult(TgFfiObjectManager manager, MemorySegment handle) {
		super(manager, handle);
	}

	public synchronized Map<TgFfiSqlCounterType, Long> getCounters(TgFfiContext context) {
		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var keysOut = allocatePtr();
		var rowsOut = allocatePtr();
		var sizeOut = allocatePtr();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_execute_result_get_counters(ctx, handle, keysOut, rowsOut,
				sizeOut);
		TgFfiRcUtil.throwIfError(rc, context);

		int size = outToInt(sizeOut);
		if (size == 0) {
			var keysArray = outToHandle(keysOut);
			var rowsArray = outToHandle(rowsOut);
			assert keysArray.address() == 0;
			assert rowsArray.address() == 0;
			return Map.of();
		} else {
			var map = new EnumMap<TgFfiSqlCounterType, Long>(TgFfiSqlCounterType.class);
			var keysArray = outToHandle(keysOut).reinterpret(ValueLayout.JAVA_INT.byteSize() * size);
			var rowsArray = outToHandle(rowsOut).reinterpret(ValueLayout.JAVA_LONG.byteSize() * size);
			for (int i = 0; i < size; i++) {
				int keyValue = keysArray.getAtIndex(ValueLayout.JAVA_INT, i);
				var key = TgFfiSqlCounterType.forNumber(keyValue);
				long rows = rowsArray.getAtIndex(ValueLayout.JAVA_LONG, i);
				map.put(key, rows);
			}
			return map;
		}
	}

//	public synchronized long getRows(TgFfiContext context, TgFfiSqlCounterType type) {
//		return switch (type) {
//		case INSERTED_ROWS -> getInsertedRows(context);
//		case UPDATED_ROWS -> getUpdatedRows(context);
//		case MERGED_ROWS -> getMergedRows(context);
//		case DELETED_ROWS -> getDeletedRows(context);
//		default -> throw new IllegalArgumentException("Unexpected value: " + type);
//		};
//	}

	public synchronized long getInsertedRows(TgFfiContext context) {
		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var out = allocatePtr();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_execute_result_get_inserted_rows(ctx, handle, out);
		TgFfiRcUtil.throwIfError(rc, context);

		return outToLong(out);
	}

	public synchronized long getUpdatedRows(TgFfiContext context) {
		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var out = allocatePtr();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_execute_result_get_updated_rows(ctx, handle, out);
		TgFfiRcUtil.throwIfError(rc, context);

		return outToLong(out);
	}

	public synchronized long getMergedRows(TgFfiContext context) {
		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var out = allocatePtr();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_execute_result_get_merged_rows(ctx, handle, out);
		TgFfiRcUtil.throwIfError(rc, context);

		return outToLong(out);
	}

	public synchronized long getDeletedRows(TgFfiContext context) {
		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var out = allocatePtr();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_execute_result_get_deleted_rows(ctx, handle, out);
		TgFfiRcUtil.throwIfError(rc, context);

		return outToLong(out);
	}

	public synchronized long getRows(TgFfiContext context) {
		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var out = allocatePtr();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_execute_result_get_rows(ctx, handle, out);
		TgFfiRcUtil.throwIfError(rc, context);

		return outToLong(out);
	}

	@Override
	protected void dispose(MemorySegment handle) {
		tsubakuro_rust_ffi_h.tsurugi_ffi_sql_execute_result_dispose(handle);
	}
}
