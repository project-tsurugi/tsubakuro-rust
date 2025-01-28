package com.tsurugidb.tsubakuro.rust.java.service.sql;

import static org.junit.jupiter.api.Assertions.*;

import java.lang.foreign.MemorySegment;
import java.util.List;

import org.junit.jupiter.api.Test;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.session.TgFfiConnectionOption;
import com.tsurugidb.tsubakuro.rust.java.session.TgFfiSession;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiTester;

class TgFfiTableListTest extends TgFfiTester {

	@Test
	void get_table_names() {
		var manager = getFfiObjectManager();
		var context = TgFfiContext.create(manager);

		dropIfExists("test");

		try (var tableList = getTableList()) {
			List<String> tableNames = tableList.getTableNames(context);
			assertFalse(tableNames.contains("test"));
		}

		executeSql("create table test (pk int primary key)");

		try (var tableList = getTableList()) {
			List<String> tableNames = tableList.getTableNames(context);
			assertTrue(tableNames.contains("test"));
		}
	}

	@Test
	void get_table_names_size_argError() {
		var manager = getFfiObjectManager();

		try (var context = TgFfiContext.create(manager)) {
			var ctx = context.handle();
			var handle = MemorySegment.NULL;
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_table_list_get_table_names_size(ctx, handle, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
		try (var context = TgFfiContext.create(manager); //
				var tableList = getTableList()) {
			var ctx = context.handle();
			var handle = tableList.handle();
			var out = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_table_list_get_table_names_size(ctx, handle, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
		}
	}

	@Test
	void get_table_names_value_argError() {
		var manager = getFfiObjectManager();

		try (var context = TgFfiContext.create(manager)) {
			var ctx = context.handle();
			var handle = MemorySegment.NULL;
			int index = 0;
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_table_list_get_table_names_value(ctx, handle, index, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
		try (var context = TgFfiContext.create(manager); //
				var tableList = getTableList()) {
			var ctx = context.handle();
			var handle = tableList.handle();
			int index = -1;
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_table_list_get_table_names_value(ctx, handle, index, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
		}
		try (var context = TgFfiContext.create(manager); //
				var tableList = getTableList()) {
			var ctx = context.handle();
			var handle = tableList.handle();
			int index = 0;
			var out = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_table_list_get_table_names_value(ctx, handle, index, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
		}
	}

	private TgFfiTableList getTableList() {
		var manager = getFfiObjectManager();

		var context = TgFfiContext.create(manager);

		var connectionOption = TgFfiConnectionOption.create(context);
		connectionOption.setEndpointUrl(context, getEndpoint());

		try (var session = TgFfiSession.connect(context, connectionOption); //
				var client = session.makeSqlClient(context)) {
			return client.listTables(context);
		}
	}
}
