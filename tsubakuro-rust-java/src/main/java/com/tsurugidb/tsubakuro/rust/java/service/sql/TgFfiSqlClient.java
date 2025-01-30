package com.tsurugidb.tsubakuro.rust.java.service.sql;

import java.lang.foreign.MemorySegment;
import java.util.List;
import java.util.Objects;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.job.TgFfiJob;
import com.tsurugidb.tsubakuro.rust.java.rc.TgFfiRcUtil;
import com.tsurugidb.tsubakuro.rust.java.service.sql.prepare.TgFfiSqlParameter;
import com.tsurugidb.tsubakuro.rust.java.service.sql.prepare.TgFfiSqlPlaceholder;
import com.tsurugidb.tsubakuro.rust.java.service.sql.prepare.TgFfiSqlPreparedStatement;
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

	public synchronized TgFfiJob<TgFfiTableList> listTablesAsync(TgFfiContext context) {
		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var out = allocatePtr();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_list_tables_async(ctx, handle, out);
		TgFfiRcUtil.throwIfError(rc, context);

		var outHandle = outToHandle(out);
		return new TgFfiJob<>(manager(), outHandle) {
			@Override
			protected TgFfiTableList valueToFfiObject(TgFfiObjectManager manager, MemorySegment valueHandle) {
				return new TgFfiTableList(manager, valueHandle);
			}
		};
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

	public synchronized TgFfiJob<TgFfiTableMetadata> getTableMetadataAsync(TgFfiContext context, String tableName) {
		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var arg = allocateString(tableName);
		var out = allocatePtr();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_get_table_metadata_async(ctx, handle, arg, out);
		TgFfiRcUtil.throwIfError(rc, context);

		var outHandle = outToHandle(out);
		return new TgFfiJob<>(manager(), outHandle) {
			@Override
			protected TgFfiTableMetadata valueToFfiObject(TgFfiObjectManager manager, MemorySegment valueHandle) {
				return new TgFfiTableMetadata(manager, valueHandle);
			}
		};
	}

	public synchronized TgFfiSqlPreparedStatement prepare(TgFfiContext context, String sql,
			List<TgFfiSqlPlaceholder> placeholders) {
		Objects.requireNonNull(sql, "sql must not be null");

		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var arg1 = allocateString(sql);
		MemorySegment arg2;
		int size;
		if (placeholders != null) {
			arg2 = allocateArray(placeholders);
			size = placeholders.size();
		} else {
			arg2 = MemorySegment.NULL;
			size = 0;
		}
		var out = allocatePtr();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_prepare(ctx, handle, arg1, arg2, size, out);
		TgFfiRcUtil.throwIfError(rc, context);

		var outHandle = outToHandle(out);
		return new TgFfiSqlPreparedStatement(manager(), outHandle);
	}

	public synchronized TgFfiJob<TgFfiSqlPreparedStatement> prepareAsync(TgFfiContext context, String sql,
			List<TgFfiSqlPlaceholder> placeholders) {
		Objects.requireNonNull(sql, "sql must not be null");

		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var arg1 = allocateString(sql);
		MemorySegment arg2;
		int size;
		if (placeholders != null) {
			arg2 = allocateArray(placeholders);
			size = placeholders.size();
		} else {
			arg2 = MemorySegment.NULL;
			size = 0;
		}
		var out = allocatePtr();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_prepare_async(ctx, handle, arg1, arg2, size, out);
		TgFfiRcUtil.throwIfError(rc, context);

		var outHandle = outToHandle(out);
		return new TgFfiJob<>(manager(), outHandle) {
			@Override
			protected TgFfiSqlPreparedStatement valueToFfiObject(TgFfiObjectManager manager,
					MemorySegment valueHandle) {
				return new TgFfiSqlPreparedStatement(manager, valueHandle);
			}
		};
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

	public synchronized TgFfiJob<TgFfiTransaction> startTransactionAsync(TgFfiContext context,
			TgFfiTransactionOption transactionOption) {
		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var arg = transactionOption.handle();
		var out = allocatePtr();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_start_transaction_async(ctx, handle, arg, out);
		TgFfiRcUtil.throwIfError(rc, context);

		var outHandle = outToHandle(out);
		return new TgFfiJob<>(manager(), outHandle) {
			@Override
			protected TgFfiTransaction valueToFfiObject(TgFfiObjectManager manager, MemorySegment valueHandle) {
				return new TgFfiTransaction(manager, valueHandle);
			}
		};
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

	public synchronized TgFfiJob<TgFfiSqlExecuteResult> executeAsync(TgFfiContext context, TgFfiTransaction transaction,
			String sql) {
		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var tx = transaction.handle();
		var arg = allocateString(sql);
		var out = allocatePtr();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_execute_async(ctx, handle, tx, arg, out);
		TgFfiRcUtil.throwIfError(rc, context);

		var outHandle = outToHandle(out);
		return new TgFfiJob<>(manager(), outHandle) {
			@Override
			protected TgFfiSqlExecuteResult valueToFfiObject(TgFfiObjectManager manager, MemorySegment valueHandle) {
				return new TgFfiSqlExecuteResult(manager, valueHandle);
			}
		};
	}

	public synchronized TgFfiSqlExecuteResult preparedExecute(TgFfiContext context, TgFfiTransaction transaction,
			TgFfiSqlPreparedStatement preparedStatement, List<TgFfiSqlParameter> parameters) {
		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var tx = transaction.handle();
		var ps = preparedStatement.handle();
		MemorySegment arg;
		int size;
		if (parameters != null) {
			arg = allocateArray(parameters);
			size = parameters.size();
		} else {
			arg = MemorySegment.NULL;
			size = 0;
		}
		var out = allocatePtr();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_prepared_execute(ctx, handle, tx, ps, arg, size, out);
		TgFfiRcUtil.throwIfError(rc, context);

		var outHandle = outToHandle(out);
		return new TgFfiSqlExecuteResult(manager(), outHandle);
	}

	public synchronized TgFfiJob<TgFfiSqlExecuteResult> preparedExecuteAsync(TgFfiContext context,
			TgFfiTransaction transaction, TgFfiSqlPreparedStatement preparedStatement,
			List<TgFfiSqlParameter> parameters) {
		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var tx = transaction.handle();
		var ps = preparedStatement.handle();
		MemorySegment arg;
		int size;
		if (parameters != null) {
			arg = allocateArray(parameters);
			size = parameters.size();
		} else {
			arg = MemorySegment.NULL;
			size = 0;
		}
		var out = allocatePtr();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_prepared_execute_async(ctx, handle, tx, ps, arg, size,
				out);
		TgFfiRcUtil.throwIfError(rc, context);

		var outHandle = outToHandle(out);
		return new TgFfiJob<>(manager(), outHandle) {
			@Override
			protected TgFfiSqlExecuteResult valueToFfiObject(TgFfiObjectManager manager, MemorySegment valueHandle) {
				return new TgFfiSqlExecuteResult(manager, valueHandle);
			}
		};
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

	public synchronized TgFfiSqlQueryResult preparedQuery(TgFfiContext context, TgFfiTransaction transaction,
			TgFfiSqlPreparedStatement preparedStatement, List<TgFfiSqlParameter> parameters) {
		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var tx = transaction.handle();
		var ps = preparedStatement.handle();
		MemorySegment arg;
		int size;
		if (parameters != null) {
			arg = allocateArray(parameters);
			size = parameters.size();
		} else {
			arg = MemorySegment.NULL;
			size = 0;
		}
		var out = allocatePtr();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_prepared_query(ctx, handle, tx, ps, arg, size, out);
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
