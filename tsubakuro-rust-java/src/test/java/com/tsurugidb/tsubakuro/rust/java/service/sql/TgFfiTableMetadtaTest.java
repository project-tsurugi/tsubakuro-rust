package com.tsurugidb.tsubakuro.rust.java.service.sql;

import static org.junit.jupiter.api.Assertions.assertEquals;

import java.lang.foreign.MemorySegment;

import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.session.TgFfiConnectionOption;
import com.tsurugidb.tsubakuro.rust.java.session.TgFfiSession;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiTester;

class TgFfiTableMetadtaTest extends TgFfiTester {

	@BeforeAll
	static void beforeAll() {
		// TODO create table test
	}

	@Test
	void get_table_name() {
		var manager = getFfiObjectManager();

		try (var tableMetadata = getTableMetadata("test")) {
			var context = TgFfiContext.create(manager);
			var tableName = tableMetadata.getTableName(context);
			assertEquals("test", tableName);
		}
	}

	@Test
	void get_columns_size_argError() {
		var manager = getFfiObjectManager();

		try (var context = TgFfiContext.create(manager)) {
			var ctx = context.handle();
			var handle = MemorySegment.NULL;
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_table_metadata_get_columns_size(ctx, handle, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
		try (var context = TgFfiContext.create(manager); //
				var tableMetadata = getTableMetadata("test")) {
			var ctx = context.handle();
			var handle = tableMetadata.handle();
			var out = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_table_metadata_get_columns_size(ctx, handle, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
		}
	}

	@Test
	void get_columns_element_argError() {
		var manager = getFfiObjectManager();

		try (var context = TgFfiContext.create(manager)) {
			var ctx = context.handle();
			var handle = MemorySegment.NULL;
			int index = 0;
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_table_metadata_get_columns_element(ctx, handle, index, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
		try (var context = TgFfiContext.create(manager); //
				var tableMetadata = getTableMetadata("test")) {
			var ctx = context.handle();
			var handle = tableMetadata.handle();
			int index = -1;
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_table_metadata_get_columns_element(ctx, handle, index, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
		}
		try (var context = TgFfiContext.create(manager); //
				var tableMetadata = getTableMetadata("test")) {
			var ctx = context.handle();
			var handle = tableMetadata.handle();
			int index = 0;
			var out = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_table_metadata_get_columns_element(ctx, handle, index, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
		}
	}

	private TgFfiTableMetadata getTableMetadata(String tableName) {
		var manager = getFfiObjectManager();

		var context = TgFfiContext.create(manager);

		var connectionOption = TgFfiConnectionOption.create(context);
		connectionOption.setEndpointUrl(context, getEndpoint());

		try (var session = TgFfiSession.connect(context, connectionOption); //
				var client = session.makeSqlClient(context)) {
			return client.getTableMetadata(context, tableName);
		}
	}
}
