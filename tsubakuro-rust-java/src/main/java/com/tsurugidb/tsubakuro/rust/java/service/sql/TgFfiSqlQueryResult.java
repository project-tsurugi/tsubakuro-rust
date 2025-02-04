package com.tsurugidb.tsubakuro.rust.java.service.sql;

import java.lang.foreign.MemorySegment;
import java.lang.foreign.ValueLayout;
import java.time.Duration;
import java.util.Objects;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.rc.TgFfiRcUtil;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObject;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObjectManager;

public class TgFfiSqlQueryResult extends TgFfiObject {

	TgFfiSqlQueryResult(TgFfiObjectManager manager, MemorySegment handle) {
		super(manager, handle);
	}

	public synchronized void setDefaultTimeout(TgFfiContext context, Duration timeout) {
		Objects.requireNonNull(timeout, "timeout must not be null");

		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var arg = allocateDuration(timeout);
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_set_default_timeout(ctx, handle, arg);
		TgFfiRcUtil.throwIfError(rc, context);
	}

	public synchronized Duration getDefaultTimeout(TgFfiContext context) {
		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var out = allocatePtr();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_get_default_timeout(ctx, handle, out);
		TgFfiRcUtil.throwIfError(rc, context);

		return outToDuration(out);
	}

	public synchronized TgFfiSqlQueryResultMetadata getMetadata(TgFfiContext context) {
		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var out = allocatePtr();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_get_metadata(ctx, handle, out);
		TgFfiRcUtil.throwIfError(rc, context);

		var outHandle = outToHandle(out);
		return new TgFfiSqlQueryResultMetadata(manager(), outHandle);
	}

	public synchronized boolean nextRow(TgFfiContext context) {
		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var out = allocatePtr();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_next_row(ctx, handle, out);
		TgFfiRcUtil.throwIfError(rc, context);

		return outToBoolean(out);
	}

	public synchronized boolean nextRowFor(TgFfiContext context, Duration timeout) {
		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var t = allocateDuration(timeout);
		var out = allocatePtr();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_next_row_for(ctx, handle, t, out);
		TgFfiRcUtil.throwIfError(rc, context);

		return outToBoolean(out);
	}

	public synchronized boolean nextColumn(TgFfiContext context) {
		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var out = allocatePtr();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_next_column(ctx, handle, out);
		TgFfiRcUtil.throwIfError(rc, context);

		return outToBoolean(out);
	}

	public synchronized boolean nextColumnFor(TgFfiContext context, Duration timeout) {
		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var t = allocateDuration(timeout);
		var out = allocatePtr();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_next_column_for(ctx, handle, t, out);
		TgFfiRcUtil.throwIfError(rc, context);

		return outToBoolean(out);
	}

	public synchronized boolean isNull(TgFfiContext context) {
		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var out = allocatePtr();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_is_null(ctx, handle, out);
		TgFfiRcUtil.throwIfError(rc, context);

		return outToBoolean(out);
	}

	public synchronized int fetchInt4(TgFfiContext context) {
		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var out = allocatePtr();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_int4(ctx, handle, out);
		TgFfiRcUtil.throwIfError(rc, context);

		return out.get(ValueLayout.JAVA_INT, 0);
	}

	public synchronized int fetchForInt4(TgFfiContext context, Duration timeout) {
		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var t = allocateDuration(timeout);
		var out = allocatePtr();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_for_int4(ctx, handle, t, out);
		TgFfiRcUtil.throwIfError(rc, context);

		return out.get(ValueLayout.JAVA_INT, 0);
	}

	public synchronized long fetchInt8(TgFfiContext context) {
		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var out = allocatePtr();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_int8(ctx, handle, out);
		TgFfiRcUtil.throwIfError(rc, context);

		return out.get(ValueLayout.JAVA_LONG, 0);
	}

	public synchronized long fetchForInt8(TgFfiContext context, Duration timeout) {
		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var t = allocateDuration(timeout);
		var out = allocatePtr();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_for_int8(ctx, handle, t, out);
		TgFfiRcUtil.throwIfError(rc, context);

		return out.get(ValueLayout.JAVA_LONG, 0);
	}

	public synchronized float fetchFloat4(TgFfiContext context) {
		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var out = allocatePtr();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_float4(ctx, handle, out);
		TgFfiRcUtil.throwIfError(rc, context);

		return out.get(ValueLayout.JAVA_FLOAT, 0);
	}

	public synchronized float fetchForFloat4(TgFfiContext context, Duration timeout) {
		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var t = allocateDuration(timeout);
		var out = allocatePtr();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_for_float4(ctx, handle, t, out);
		TgFfiRcUtil.throwIfError(rc, context);

		return out.get(ValueLayout.JAVA_FLOAT, 0);
	}

	public synchronized double fetchFloat8(TgFfiContext context) {
		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var out = allocatePtr();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_float8(ctx, handle, out);
		TgFfiRcUtil.throwIfError(rc, context);

		return out.get(ValueLayout.JAVA_DOUBLE, 0);
	}

	public synchronized double fetchForFloat8(TgFfiContext context, Duration timeout) {
		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var t = allocateDuration(timeout);
		var out = allocatePtr();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_for_float8(ctx, handle, t, out);
		TgFfiRcUtil.throwIfError(rc, context);

		return out.get(ValueLayout.JAVA_DOUBLE, 0);
	}

	public synchronized String fetchCharacter(TgFfiContext context) {
		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var out = allocatePtr();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_character(ctx, handle, out);
		TgFfiRcUtil.throwIfError(rc, context);

		return outToString(out);
	}

	public synchronized String fetchForCharacter(TgFfiContext context, Duration timeout) {
		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var t = allocateDuration(timeout);
		var out = allocatePtr();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_for_character(ctx, handle, t, out);
		TgFfiRcUtil.throwIfError(rc, context);

		return outToString(out);
	}

	@Override
	protected void dispose(MemorySegment handle) {
		tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_dispose(handle);
	}
}
