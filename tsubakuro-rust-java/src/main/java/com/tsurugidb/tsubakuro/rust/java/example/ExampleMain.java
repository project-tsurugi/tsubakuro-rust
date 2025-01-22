package com.tsurugidb.tsubakuro.rust.java.example;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;

public class ExampleMain {

	public static void main(String[] args) {
		var ffiLibraryPath = args[0];
		System.load(ffiLibraryPath);

		int rc = tsubakuro_rust_ffi_h.tsurugi_ffi_env_logger_init();
		System.out.println("tsurugi_ffi_env_logger_init() rc=" + rc);
	}
}
