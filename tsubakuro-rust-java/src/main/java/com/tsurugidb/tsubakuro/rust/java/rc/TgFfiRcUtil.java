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

    public static void throwIfError(int rc) {
        if (!isOk(rc)) {
            throw new TgFfiRuntimeException(rc);
        }
    }

    public static void throwIfError(int rc, TgFfiContext context) {
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

        if ((rc & 0xc000_0000) == tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_CORE_SERVER_ERROR()) {
            return "SERVER_ERROR";
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
        map.put(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), "FFI_ARG3_ERROR");
        map.put(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG4_ERROR(), "FFI_ARG4_ERROR");
        map.put(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG5_ERROR(), "FFI_ARG5_ERROR");
        map.put(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG6_ERROR(), "FFI_ARG6_ERROR");
        map.put(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG7_ERROR(), "FFI_ARG7_ERROR");
        map.put(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_JOB_ALREADY_CLOSED(), "FFI_JOB_ALREADY_CLOSED");
        map.put(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_NUL_ERROR(), "FFI_NUL_ERROR");
        map.put(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_DIAGNOSTIC_CODE_NOT_FOUND(), "FFI_DIAGNOSTIC_CODE_NOT_FOUND");
        map.put(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_CORE_CLIENT_CLIENT_ERROR(), "CLIENT_ERROR");
        map.put(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_CORE_CLIENT_TIMEOUT_ERROR(), "TIMEOUT_ERROR");
        map.put(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_CORE_CLIENT_IO_ERROR(), "IO_ERROR");
        NAME_MAP = map;

        return map;
    }
}
