package com.tsurugidb.tsubakuro.rust.java.rc;

import java.util.HashMap;
import java.util.Map;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiRuntimeException;

public class TgFfiRcUtil {

	public static boolean isOk(int rc) {
		return rc == tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_OK();
	}

	public static void throwIfNg(int rc) {
		if (!isOk(rc)) {
			throw new TgFfiRuntimeException(rc);
		}
	}

	public static void throwIfNg(int rc, TgFfiContext context) {
		if (!isOk(rc)) {
			throw new TgFfiRuntimeException(rc, context);
		}
	}

	public static String toName(int rc) {
		var nameMap = nameMap();
		String name = nameMap.get(rc);
		if (name != null) {
			return name;
		}
		return String.format("Rc%08x", rc);
	}

	private static Map<Integer, String> NAME_MAP;

	private static Map<Integer, String> nameMap() {
		if (NAME_MAP != null) {
			return NAME_MAP;
		}

		var map = new HashMap<Integer, String>();
		map.put(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_OK(), "OK");
		map.put(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG0_ERROR(), "FFI_ARG0_ERROR");
		map.put(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), "FFI_ARG1_ERROR");
		map.put(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), "FFI_ARG2_ERROR");
		map.put(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_NUL_ERROR(), "FFI_NUL_ERROR");
		NAME_MAP = map;
		return map;
	}
}
