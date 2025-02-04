package com.tsurugidb.tsubakuro.rust.java.job;

import java.lang.foreign.MemorySegment;
import java.time.Duration;
import java.util.Optional;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.rc.TgFfiRcUtil;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObject;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObjectManager;

public abstract class TgFfiJob<T> extends TgFfiObject {

	public TgFfiJob(TgFfiObjectManager manager, MemorySegment handle) {
		super(manager, handle);
	}

	public synchronized String getName(TgFfiContext context) {
		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var out = allocatePtr();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_job_get_name(ctx, handle, out);
		TgFfiRcUtil.throwIfError(rc, context);

		return outToString(out);
	}

	public synchronized boolean wait(TgFfiContext context, Duration timeout) {
		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var out = allocatePtr();
		var t = allocateDuration(timeout);
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_job_wait(ctx, handle, t, out);
		TgFfiRcUtil.throwIfError(rc, context);

		return outToBoolean(out);
	}

	public synchronized boolean isDone(TgFfiContext context) {
		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var out = allocatePtr();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_job_is_done(ctx, handle, out);
		TgFfiRcUtil.throwIfError(rc, context);

		return outToBoolean(out);
	}

	public synchronized T take(TgFfiContext context) {
		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var out = allocatePtr();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_job_take(ctx, handle, out);
		TgFfiRcUtil.throwIfError(rc, context);

		var outHandle = outToHandle(out);
		return valueToFfiObject(manager(), outHandle);
	}

	public synchronized T takeFor(TgFfiContext context, Duration timeout) {
		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var out = allocatePtr();
		var t = allocateDuration(timeout);
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_job_take_for(ctx, handle, t, out);
		TgFfiRcUtil.throwIfError(rc, context);

		var outHandle = outToHandle(out);
		return valueToFfiObject(manager(), outHandle);
	}

	public synchronized Optional<T> takeIfReady(TgFfiContext context) {
		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var isReadyOut = allocatePtr();
		var out = allocatePtr();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_job_take_if_ready(ctx, handle, isReadyOut, out);
		TgFfiRcUtil.throwIfError(rc, context);

		var isReady = outToBoolean(isReadyOut);
		var outHandle = outToHandle(out);
		if (!isReady) {
			assert outHandle == null;
			return null;
		}

		if (outHandle.address() == 0) {
			return Optional.empty();
		}
		return Optional.ofNullable(valueToFfiObject(manager(), outHandle));
	}

	public synchronized boolean cancel(TgFfiContext context) {
		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var out = allocatePtr();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_job_cancel(ctx, handle, out);
		TgFfiRcUtil.throwIfError(rc, context);

		return outToBoolean(out);
	}

	public synchronized boolean cancelFor(TgFfiContext context, Duration timeout) {
		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var t = allocateDuration(timeout);
		var out = allocatePtr();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_job_cancel_for(ctx, handle, t, out);
		TgFfiRcUtil.throwIfError(rc, context);

		return outToBoolean(out);
	}

	public synchronized TgFfiCancelJob cancelAsync(TgFfiContext context) {
		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var out = allocatePtr();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_job_cancel_async(ctx, handle, out);
		TgFfiRcUtil.throwIfError(rc, context);

		var outHandle = outToHandle(out);
		if (outHandle.address() == 0) {
			return null;
		}
		return new TgFfiCancelJob(manager(), outHandle);
	}

	public synchronized void close(TgFfiContext context) {
		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_job_close(ctx, handle);
		TgFfiRcUtil.throwIfError(rc, context);
	}

	protected abstract T valueToFfiObject(TgFfiObjectManager manager, MemorySegment valueHandle);

	@Override
	protected void dispose(MemorySegment handle) {
		tsubakuro_rust_ffi_h.tsurugi_ffi_job_dispose(handle);
	}
}
