package com.tsurugidb.tsubakuro.rust.java.type;

import static org.junit.jupiter.api.Assertions.assertArrayEquals;
import static org.junit.jupiter.api.Assertions.assertEquals;

import java.time.Duration;
import java.util.ArrayList;
import java.util.List;

import com.tsurugidb.iceaxe.sql.parameter.TgBindParameter;
import com.tsurugidb.iceaxe.sql.parameter.TgBindVariable;
import com.tsurugidb.iceaxe.sql.result.TsurugiResultEntity;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.service.sql.TgFfiAtomType;
import com.tsurugidb.tsubakuro.rust.java.service.sql.TgFfiSqlQueryResult;
import com.tsurugidb.tsubakuro.rust.java.service.sql.prepare.TgFfiSqlParameter;

class TgFfiTypeOctetTest extends TgFfiTypeTester<byte[]> {

    @Override
    protected String sqlType() {
        return "varbinary";
    }

    @Override
    protected String expectedSqlType() {
        return "VARBINARY(*)";
    }

    @Override
    protected List<byte[]> values() {
        var list = new ArrayList<byte[]>();
        list.add(new byte[0]);
        list.add(new byte[] { 1, 2, 3, 100, (byte) 0xff });
        list.add(null);
        return list;
    }

    @Override
    protected TgBindVariable<byte[]> bindVariable(String name) {
        return TgBindVariable.ofBytes(name);
    }

    @Override
    protected TgBindParameter bindParameter(String name, byte[] value) {
        return TgBindParameter.of(name, value);
    }

    @Override
    protected byte[] get(TsurugiResultEntity entity, String name) {
        return entity.getBytes(name);
    }

    @Override
    protected TgFfiAtomType ffiAtomType() {
        return TgFfiAtomType.OCTET;
    }

    @Override
    protected TgFfiSqlParameter ffiParameter(TgFfiContext context, String name, byte[] value) {
        return TgFfiSqlParameter.ofOctet(context, name, value);
    }

    @Override
    protected byte[] fetch(TgFfiContext context, TgFfiSqlQueryResult qr) {
        return qr.fetchOctet(context);
    }

    @Override
    protected byte[] fetchFor(TgFfiContext context, TgFfiSqlQueryResult qr, Duration timeout) {
        return qr.fetchForOctet(context, timeout);
    }

    @Override
    protected void assertValueList(List<byte[]> expected, List<byte[]> actual) {
        assertEquals(expected.size(), actual.size());
        for (int i = 0; i < actual.size(); i++) {
            assertArrayEquals(expected.get(i), actual.get(i));
        }
    }
}
