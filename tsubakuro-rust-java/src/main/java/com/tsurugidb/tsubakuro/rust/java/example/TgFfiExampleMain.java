package com.tsurugidb.tsubakuro.rust.java.example;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.session.TgFfiEndpoint;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObjectManager;

public class TgFfiExampleMain {

	public static void main(String[] args) {
		var ffiLibraryPath = args[0];
		System.load(ffiLibraryPath);

		int rc = tsubakuro_rust_ffi_h.tsurugi_ffi_env_logger_init();
		System.out.println("tsurugi_ffi_env_logger_init() rc=" + rc);

		try (var manager = TgFfiObjectManager.create()) {
			var context = TgFfiContext.create(manager);
			var endpoint = TgFfiEndpoint.parse(context, "tcp://localhost:12345");
		}
	}
}
