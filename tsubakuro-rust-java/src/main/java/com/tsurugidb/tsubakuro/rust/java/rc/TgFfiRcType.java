package com.tsurugidb.tsubakuro.rust.java.rc;

import java.util.Map;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;

public enum TgFfiRcType {
	OK, FFI_ERROR, CORE_CLIENT_ERROR, CORE_SERVER_ERROR;

	private static final Map<Integer, TgFfiRcType> TYPE_MAP = Map.of( //
			tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_TYPE_OK(), TgFfiRcType.OK, //
			tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_TYPE_FFI_ERROR(), TgFfiRcType.FFI_ERROR, //
			tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_TYPE_CORE_CLIENT_ERROR(), TgFfiRcType.CORE_CLIENT_ERROR, //
			tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_TYPE_CORE_SERVER_ERROR(), TgFfiRcType.CORE_SERVER_ERROR);

	public static TgFfiRcType of(int rcType) {
		var type = TYPE_MAP.get(rcType);
		if (type == null) {
			throw new AssertionError("unknown rcType. rcType=" + rcType);
		}
		return type;
	}
}
