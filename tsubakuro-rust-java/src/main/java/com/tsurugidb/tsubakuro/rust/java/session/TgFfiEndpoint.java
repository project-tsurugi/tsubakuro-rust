package com.tsurugidb.tsubakuro.rust.java.session;

import java.lang.foreign.MemorySegment;
import java.util.Objects;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObject;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObjectManager;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiRcUtil;

public class TgFfiEndpoint extends TgFfiObject {

	public static TgFfiEndpoint parse(TgFfiContext context, String endpoint) {
		Objects.requireNonNull(context, "context must not be null");
		return parse(context.manager(), context, endpoint);
	}

	public static TgFfiEndpoint parse(TgFfiObjectManager manager, String endpoint) {
		return parse(manager, null, endpoint);
	}

	public static TgFfiEndpoint parse(TgFfiObjectManager manager, TgFfiContext context, String endpoint) {
		Objects.requireNonNull(manager, "manager must not be null");
		Objects.requireNonNull(endpoint, "endpoint must not be null");

		if (context != null) {
			synchronized (context) {
				return parseMain(manager, context, endpoint);
			}
		} else {
			return parseMain(manager, null, endpoint);
		}
	}

	private static TgFfiEndpoint parseMain(TgFfiObjectManager manager, TgFfiContext context, String endpoint) {
		var contextHandle = (context != null) ? context.handle() : MemorySegment.NULL;
		var endpointHandle = manager.allocateString(endpoint);
		var out = manager.allocatePtr();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_endpoint_parse(contextHandle, endpointHandle, out);
		TgFfiRcUtil.throwIfNg(rc);

		var handle = outToHandle(out);
		return new TgFfiEndpoint(manager, handle);
	}

	TgFfiEndpoint(TgFfiObjectManager manager, MemorySegment handle) {
		super(manager, handle);
	}

	@Override
	protected void dispose(MemorySegment handle) {
		tsubakuro_rust_ffi_h.tsurugi_ffi_endpoint_dispose(handle);
	}
}
