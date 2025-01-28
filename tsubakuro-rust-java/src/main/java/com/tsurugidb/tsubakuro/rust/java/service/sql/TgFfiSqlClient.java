package com.tsurugidb.tsubakuro.rust.java.service.sql;

import java.lang.foreign.MemorySegment;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.rc.TgFfiRcUtil;
import com.tsurugidb.tsubakuro.rust.java.transaction.TgFfiCommitOption;
import com.tsurugidb.tsubakuro.rust.java.transaction.TgFfiTransaction;
import com.tsurugidb.tsubakuro.rust.java.transaction.TgFfiTransactionOption;
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

		var outHandle = outToHandle(out);
		return new TgFfiTableList(manager(), outHandle);
	}

	public synchronized TgFfiTableMetadata getTableMetadata(TgFfiContext context, String tableName) {
		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var arg = allocateString(tableName);
		var out = allocatePtr();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_get_table_metadata(ctx, handle, arg, out);
		TgFfiRcUtil.throwIfError(rc, context);

		var outHandle = outToHandle(out);
		return new TgFfiTableMetadata(manager(), outHandle);
	}

	public synchronized TgFfiTransaction startTransaction(TgFfiContext context,
			TgFfiTransactionOption transactionOption) {
		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var arg = transactionOption.handle();
		var out = allocatePtr();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_start_transaction(ctx, handle, arg, out);
		TgFfiRcUtil.throwIfError(rc, context);

		var outHandle = outToHandle(out);
		return new TgFfiTransaction(manager(), outHandle);
	}

	public synchronized TgFfiSqlExecuteResult execute(TgFfiContext context, TgFfiTransaction transaction, String sql) {
		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var tx = transaction.handle();
		var arg = allocateString(sql);
		var out = allocatePtr();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_execute(ctx, handle, tx, arg, out);
		TgFfiRcUtil.throwIfError(rc, context);

		var outHandle = outToHandle(out);
		return new TgFfiSqlExecuteResult(manager(), outHandle);
	}

	public synchronized TgFfiSqlQueryResult query(TgFfiContext context, TgFfiTransaction transaction, String sql) {
		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var tx = transaction.handle();
		var arg = allocateString(sql);
		var out = allocatePtr();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_query(ctx, handle, tx, arg, out);
		TgFfiRcUtil.throwIfError(rc, context);

		var outHandle = outToHandle(out);
		return new TgFfiSqlQueryResult(manager(), outHandle);
	}

	public synchronized void commit(TgFfiContext context, TgFfiTransaction transaction,
			TgFfiCommitOption commitOption) {
		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var tx = transaction.handle();
		var arg = commitOption.handle();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_commit(ctx, handle, tx, arg);
		TgFfiRcUtil.throwIfError(rc, context);
	}

	public synchronized void rollback(TgFfiContext context, TgFfiTransaction transaction) {
		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var tx = transaction.handle();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_rollback(ctx, handle, tx);
		TgFfiRcUtil.throwIfError(rc, context);
	}

	@Override
	protected void dispose(MemorySegment handle) {
		tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_dispose(handle);
	}
}
