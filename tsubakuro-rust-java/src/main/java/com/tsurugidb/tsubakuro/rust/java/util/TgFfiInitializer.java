package com.tsurugidb.tsubakuro.rust.java.util;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;

public class TgFfiInitializer {

    public static void loadFfiLibrary() {
        String path = System.getProperty("tsurugi.ffi.library.path");
        if (path != null && !path.isEmpty()) {
            loadFfiLibrary(path);
            return;
        }

        String pathEnv = System.getenv("TSURUGI_FFI_LIBRARY_PATH");
        if (pathEnv != null && !pathEnv.isEmpty()) {
            loadFfiLibrary(pathEnv);
            return;
        }

        throw new RuntimeException("-Dtsurugi.ffi.library.path or TSURUGI_FFI_LIBRARY_PATH not defined or empty");
    }

    public static void loadFfiLibrary(String path) {
        System.load(path);
    }

    public static int initFfiEnvLogger() {
        return tsubakuro_rust_ffi_h.tsurugi_ffi_env_logger_init();
    }

    public static int initFfiEnvLogger(String filters, String filePath) {
        try (var manager = TgFfiObjectManager.create()) {
            var filtersMemory = manager.allocateString(filters);
            var filePathMemory = manager.allocateString(filePath);
            return tsubakuro_rust_ffi_h.tsurugi_ffi_env_logger_init_with_filters(filtersMemory, filePathMemory);
        }
    }
}
