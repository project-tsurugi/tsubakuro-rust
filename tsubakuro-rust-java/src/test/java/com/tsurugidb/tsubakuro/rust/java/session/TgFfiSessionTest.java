package com.tsurugidb.tsubakuro.rust.java.session;

import static org.junit.jupiter.api.Assertions.assertEquals;

import java.lang.foreign.MemorySegment;
import java.time.Duration;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiTester;

class TgFfiSessionTest extends TgFfiTester {

	@Test
	void connect() {
		var manager = getFfiObjectManager();
		var context = TgFfiContext.create(manager);

		var connectionOption = TgFfiConnectionOption.create(context);
		connectionOption.setEndpointUrl(context, getEndpoint());

		try (var session = TgFfiSession.connect(context, connectionOption)) {
		}
	}

	@Test
	void connect_argError() {
		var manager = getFfiObjectManager();
		var context = TgFfiContext.create(manager);

		{
			var ctx = context.handle();
			var arg = MemorySegment.NULL;
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_session_connect(ctx, arg, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
		try (var connectionOption = TgFfiConnectionOption.create(context)) {
			var ctx = context.handle();
			var arg = connectionOption.handle();
			var out = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_session_connect(ctx, arg, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
		}
	}

	@Test
	void connect_for() {
		var manager = getFfiObjectManager();
		var context = TgFfiContext.create(manager);

		var connect_forionOption = TgFfiConnectionOption.create(context);
		connect_forionOption.setEndpointUrl(context, getEndpoint());

		var timeout = Duration.ofSeconds(5);

		try (var session = TgFfiSession.connectFor(context, connect_forionOption, timeout)) {
		}
	}

	@Test
	void connect_for_argError() {
		var manager = getFfiObjectManager();
		var context = TgFfiContext.create(manager);

		{
			var ctx = context.handle();
			var arg = MemorySegment.NULL;
			var t = Duration.ofSeconds(5).toNanos();
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_session_connect_for(ctx, arg, t, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
		try (var connect_forionOption = TgFfiConnectionOption.create(context)) {
			var ctx = context.handle();
			var arg = connect_forionOption.handle();
			var t = Duration.ofSeconds(5).toNanos();
			var out = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_session_connect_for(ctx, arg, t, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
		}
	}

	@ParameterizedTest
	@ValueSource(strings = { TAKE, TAKE_FOR, TAKE_IF_READY })
	void connect_async(String pattern) {
		var manager = getFfiObjectManager();
		var context = TgFfiContext.create(manager);

		var connectionOption = TgFfiConnectionOption.create(context);
		connectionOption.setEndpointUrl(context, getEndpoint());

		try (var sessionJob = TgFfiSession.connectAsync(context, connectionOption); //
				var session = jobTake(sessionJob, pattern)) {
		}
	}

	@Test
	void connect_async_argError() {
		var manager = getFfiObjectManager();
		var context = TgFfiContext.create(manager);

		{
			var ctx = context.handle();
			var arg = MemorySegment.NULL;
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_session_connect_async(ctx, arg, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
		try (var connectionOption = TgFfiConnectionOption.create(context)) {
			var ctx = context.handle();
			var arg = connectionOption.handle();
			var out = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_session_connect_async(ctx, arg, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
		}
	}

	@Test
	void set_default_timeout() {
		var manager = getFfiObjectManager();
		var context = TgFfiContext.create(manager);

		var connectionOption = TgFfiConnectionOption.create(context);
		connectionOption.setEndpointUrl(context, getEndpoint());

		try (var session = TgFfiSession.connect(context, connectionOption)) {
			session.setDefaultTimeout(context, Duration.ofSeconds(123));

			var timeout = session.getDefaultTimeout(context);
			assertEquals(Duration.ofSeconds(123), timeout);
		}
	}

	@Test
	void set_default_timeout_argError() {
		var manager = getFfiObjectManager();
		var context = TgFfiContext.create(manager);

		{
			var ctx = context.handle();
			var handle = MemorySegment.NULL;
			var arg = Duration.ofSeconds(5).toNanos();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_session_set_default_timeout(ctx, handle, arg);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
	}

	@Test
	void get_default_timeout_argError() {
		var manager = getFfiObjectManager();
		var context = TgFfiContext.create(manager);

		{
			var ctx = context.handle();
			var handle = MemorySegment.NULL;
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_session_get_default_timeout(ctx, handle, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
		try (var session = createSession()) {
			var ctx = context.handle();
			var handle = session.handle();
			var out = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_session_get_default_timeout(ctx, handle, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
		}
	}

	@Test
	void make_sql_client() {
		var manager = getFfiObjectManager();
		var context = TgFfiContext.create(manager);

		var connectionOption = TgFfiConnectionOption.create(context);
		connectionOption.setEndpointUrl(context, getEndpoint());

		try (var session = TgFfiSession.connect(context, connectionOption); //
				var client = session.makeSqlClient(context)) {
		}
	}

	@Test
	void make_sql_client_argError() {
		var manager = getFfiObjectManager();
		var context = TgFfiContext.create(manager);

		{
			var ctx = context.handle();
			var handle = MemorySegment.NULL;
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_session_make_sql_client(ctx, handle, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
		{
			var ctx = context.handle();
			var handle = manager.allocateString("tcp://localhost:12345");
			var out = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_session_make_sql_client(ctx, handle, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
		}
	}
}
