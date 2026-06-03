package com.tsurugidb.tsubakuro.rust.java.util;

import java.nio.file.Path;

public record TgFfiPathMappingEntry(Path clientPath, String serverPath) {

    /**
     * Parses a path mapping string and creates a new instance.
     *
     * @param pathMapping path mapping string
     * @return a new instance
     */
    public static TgFfiPathMappingEntry parse(String pathMapping) {
        int n = pathMapping.lastIndexOf(':');
        if (n < 0) {
            throw new IllegalArgumentException("Invalid path mapping: " + pathMapping);
        }
        String clientPath = pathMapping.substring(0, n).trim();
        String serverPath = pathMapping.substring(n + 1).trim();

        if (clientPath.isEmpty() || serverPath.isEmpty()) {
            throw new IllegalArgumentException("Invalid path mapping, both client and server paths must be non-empty: " + pathMapping);
        }

        return new TgFfiPathMappingEntry(Path.of(clientPath), serverPath);
    }
}
