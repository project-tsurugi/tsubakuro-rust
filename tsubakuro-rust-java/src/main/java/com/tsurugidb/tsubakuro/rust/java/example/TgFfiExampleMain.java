package com.tsurugidb.tsubakuro.rust.java.example;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.session.TgFfiConnectionOption;
import com.tsurugidb.tsubakuro.rust.java.session.TgFfiSession;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObjectManager;

public class TgFfiExampleMain {

	public static void main(String[] args) throws Exception {
		var ffiLibraryPath = args[0];
		System.load(ffiLibraryPath);

		int rc = tsubakuro_rust_ffi_h.tsurugi_ffi_env_logger_init();
		System.out.println("tsurugi_ffi_env_logger_init() rc=" + rc);

		try (var manager = TgFfiObjectManager.create()) {
			var context = TgFfiContext.create(manager);

			var connectionOption = TgFfiConnectionOption.create(context);
			connectionOption.setEndpointUrl(context, "tcp://localhost:12345");

			try (var session = TgFfiSession.connect(context, connectionOption)) {

			}
		}
	}
}
