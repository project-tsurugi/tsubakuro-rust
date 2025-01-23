package com.tsurugidb.tsubakuro.rust.java.session;

import java.lang.foreign.MemorySegment;
import java.util.Objects;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.rc.TgFfiRcUtil;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObject;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObjectManager;

public class TgFfiConnectionOption extends TgFfiObject {

	public static TgFfiConnectionOption create(TgFfiContext context) {
		Objects.requireNonNull(context, "context must not be null");
		return create(context.manager(), context);
	}

	public static TgFfiConnectionOption create(TgFfiObjectManager manager) {
		return create(manager, null);
	}

	public static TgFfiConnectionOption create(TgFfiObjectManager manager, TgFfiContext context) {
		Objects.requireNonNull(manager, "manager must not be null");

		if (context != null) {
			synchronized (context) {
				return createMain(manager, context);
			}
		} else {
			return createMain(manager, null);
		}
	}

	private static TgFfiConnectionOption createMain(TgFfiObjectManager manager, TgFfiContext context) {
		var contextHandle = (context != null) ? context.handle() : MemorySegment.NULL;
		var out = manager.allocatePtr();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_create(contextHandle, out);
		TgFfiRcUtil.throwIfNg(rc, context);

		var handle = outToHandle(out);
		return new TgFfiConnectionOption(manager, handle);
	}

	TgFfiConnectionOption(TgFfiObjectManager manager, MemorySegment handle) {
		super(manager, handle);
	}

	public synchronized void setEndpoint(TgFfiContext context, TgFfiEndpoint endpoint) {
		Objects.requireNonNull(endpoint, "endpoint must not be null");

		var contextHandle = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var endpointHandle = endpoint.handle();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_set_endpoint(contextHandle, handle, endpointHandle);
		TgFfiRcUtil.throwIfNg(rc, context);
	}

	public synchronized void setEndpointUrl(TgFfiContext context, String endpoint) {
		Objects.requireNonNull(endpoint, "endpoint must not be null");

		var contextHandle = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var endpointHandle = allocateString(endpoint);
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_set_endpoint_url(contextHandle, handle,
				endpointHandle);
		TgFfiRcUtil.throwIfNg(rc, context);
	}

	public synchronized String getEndpoint(TgFfiContext context) {
		var contextHandle = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var out = allocatePtr();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_get_endpoint(contextHandle, handle, out);
		TgFfiRcUtil.throwIfNg(rc, context);

		return outToString(out);
	}

	@Override
	protected void dispose(MemorySegment handle) {
		tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_dispose(handle);
	}
}
