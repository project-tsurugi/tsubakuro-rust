package com.tsurugidb.tsubakuro.rust.java.util;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;

public class TgFfiRcUtil {

	public static boolean isOk(int rc) {
		return rc == tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_OK();
	}

	public static void throwIfNg(int rc) {
		if (!isOk(rc)) {
			throw new RuntimeException("rc=" + rc); // TODO TgFfiRuntimeException
		}
	}
}
