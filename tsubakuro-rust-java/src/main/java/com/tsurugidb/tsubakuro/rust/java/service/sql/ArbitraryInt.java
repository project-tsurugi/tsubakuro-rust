package com.tsurugidb.tsubakuro.rust.java.service.sql;

public record ArbitraryInt(int value, boolean arbitrary) {

    public static ArbitraryInt of(int value) {
        return new ArbitraryInt(value, false);
    }

    public static ArbitraryInt ofArbitrary() {
        return new ArbitraryInt(0, true);
    }
}
