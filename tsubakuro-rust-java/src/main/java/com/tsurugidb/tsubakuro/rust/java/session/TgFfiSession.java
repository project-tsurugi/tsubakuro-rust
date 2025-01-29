package com.tsurugidb.tsubakuro.rust.java.session;

import java.lang.foreign.MemorySegment;
import java.util.Objects;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.job.TgFfiJob;
import com.tsurugidb.tsubakuro.rust.java.rc.TgFfiRcUtil;
import com.tsurugidb.tsubakuro.rust.java.service.sql.TgFfiSqlClient;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObject;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObjectManager;

public class TgFfiSession extends TgFfiObject {

	public static TgFfiSession connect(TgFfiContext context, TgFfiConnectionOption connectionOption) {
		Objects.requireNonNull(context, "context must not be null");
		return connect(context.manager(), context, connectionOption);
	}

	public static TgFfiSession connect(TgFfiObjectManager manager, TgFfiConnectionOption connectionOption) {
		return connect(manager, null, connectionOption);
	}

	public static TgFfiSession connect(TgFfiObjectManager manager, TgFfiContext context,
			TgFfiConnectionOption connectionOption) {
		Objects.requireNonNull(manager, "manager must not be null");
		Objects.requireNonNull(connectionOption, "connectionOption must not be null");

		if (context != null) {
			synchronized (context) {
				return connectMain(manager, context, connectionOption);
			}
		} else {
			return connectMain(manager, null, connectionOption);
		}
	}

	private static TgFfiSession connectMain(TgFfiObjectManager manager, TgFfiContext context,
			TgFfiConnectionOption connectionOption) {
		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var arg = connectionOption.handle();
		var out = manager.allocatePtr();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_session_connect(ctx, arg, out);
		TgFfiRcUtil.throwIfError(rc, context);

		var outHandle = outToHandle(out);
		return new TgFfiSession(manager, outHandle);
	}

	public static TgFfiJob<TgFfiSession> connectAsync(TgFfiContext context, TgFfiConnectionOption connectionOption) {
		Objects.requireNonNull(context, "context must not be null");
		return connectAsync(context.manager(), context, connectionOption);
	}

	public static TgFfiJob<TgFfiSession> connectAsync(TgFfiObjectManager manager,
			TgFfiConnectionOption connectionOption) {
		return connectAsync(manager, null, connectionOption);
	}

	public static TgFfiJob<TgFfiSession> connectAsync(TgFfiObjectManager manager, TgFfiContext context,
			TgFfiConnectionOption connectionOption) {
		Objects.requireNonNull(manager, "manager must not be null");
		Objects.requireNonNull(connectionOption, "connectOption must not be null");

		if (context != null) {
			synchronized (context) {
				return connectAsyncMain(manager, context, connectionOption);
			}
		} else {
			return connectAsyncMain(manager, null, connectionOption);
		}
	}

	private static TgFfiJob<TgFfiSession> connectAsyncMain(TgFfiObjectManager manager, TgFfiContext context,
			TgFfiConnectionOption connectionOption) {
		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var arg = connectionOption.handle();
		var out = manager.allocatePtr();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_session_connect_async(ctx, arg, out);
		TgFfiRcUtil.throwIfError(rc, context);

		var outHandle = outToHandle(out);
		return new TgFfiJob<>(manager, outHandle) {
			@Override
			protected TgFfiSession valueToFfiObject(TgFfiObjectManager manager, MemorySegment valueHandle) {
				return new TgFfiSession(manager, valueHandle);
			}
		};
	}

	TgFfiSession(TgFfiObjectManager manager, MemorySegment handle) {
		super(manager, handle);
	}

	public synchronized TgFfiSqlClient makeSqlClient(TgFfiContext context) {
		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var out = allocatePtr();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_session_make_sql_client(ctx, handle, out);
		TgFfiRcUtil.throwIfError(rc, context);

		var outHandle = outToHandle(out);
		return new TgFfiSqlClient(manager(), outHandle);
	}

	@Override
	protected void dispose(MemorySegment handle) {
		tsubakuro_rust_ffi_h.tsurugi_ffi_session_dispose(handle);
	}
}
