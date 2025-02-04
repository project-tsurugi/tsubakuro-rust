package com.tsurugidb.tsubakuro.rust.java.transaction;

import java.lang.foreign.MemorySegment;
import java.time.Duration;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.rc.TgFfiRcUtil;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObject;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObjectManager;

public class TgFfiTransaction extends TgFfiObject {

	public TgFfiTransaction(TgFfiObjectManager manager, MemorySegment handle) {
		super(manager, handle);
	}

	public synchronized String getTransactionId(TgFfiContext context) {
		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var out = allocatePtr();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_get_transaction_id(ctx, handle, out);
		TgFfiRcUtil.throwIfError(rc, context);

		return outToString(out);
	}

	public synchronized void close(TgFfiContext context) {
		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_close(ctx, handle);
		TgFfiRcUtil.throwIfError(rc, context);
	}

	public synchronized void closeFor(TgFfiContext context, Duration timeout) {
		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var t = allocateDuration(timeout);
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_close_for(ctx, handle, t);
		TgFfiRcUtil.throwIfError(rc, context);
	}

	@Override
	public void close() {
		RuntimeException re = null;
		try {
			close(null);
		} catch (RuntimeException e) {
			re = e;
		}

		try {
			super.close();
		} catch (RuntimeException e) {
			if (re != null) {
				e.addSuppressed(re);
			}
			throw e;
		}

		if (re != null) {
			throw re;
		}
	}

	@Override
	protected void dispose(MemorySegment handle) {
		tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_dispose(handle);
	}
}
