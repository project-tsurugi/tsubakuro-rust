package com.tsurugidb.tsubakuro.rust.java.example;

import java.lang.foreign.Arena;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;

public class ExampleMain {

	public static void main(String[] args) {
		var ffiLibraryPath = args[0];
		System.load(ffiLibraryPath);

		int rc = tsubakuro_rust_ffi_h.tsurugi_ffi_env_logger_init();
		System.out.println("tsurugi_ffi_env_logger_init() rc=" + rc);

		try (var arena = Arena.ofConfined()) {
			try (var context = TgFfiContext.create(arena)) {
			}
		}
	}
}
