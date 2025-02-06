package com.tsurugidb.tsubakuro.rust.java.type;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNull;

import java.math.BigDecimal;
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

class TgFfiTypeDecimalTest extends TgFfiTypeTester<BigDecimal> {

    @Override
    protected String sqlType() {
        return "decimal(38, 1)";
    }

    @Override
    protected List<BigDecimal> values() {
        var list = new ArrayList<BigDecimal>();
        list.add(new BigDecimal("-" + "9".repeat(37) + ".9"));
        list.add(BigDecimal.valueOf(-1));
        list.add(BigDecimal.valueOf(0));
        list.add(BigDecimal.valueOf(1));
        list.add(new BigDecimal("123.4"));
        list.add(new BigDecimal("9".repeat(37) + ".9"));
        list.add(null);
        return list;
    }

    @Override
    protected TgBindVariable<BigDecimal> bindVariable(String name) {
        return TgBindVariable.ofDecimal(name);
    }

    @Override
    protected TgBindParameter bindParameter(String name, BigDecimal value) {
        return TgBindParameter.of(name, value);
    }

    @Override
    protected BigDecimal get(TsurugiResultEntity entity, String name) {
        return entity.getDecimal(name);
    }

    @Override
    protected TgFfiAtomType ffiAtomType() {
        return TgFfiAtomType.DECIMAL;
    }

    @Override
    protected TgFfiSqlParameter ffiParameter(TgFfiContext context, String name, BigDecimal value) {
        return TgFfiSqlParameter.ofDecimal(context, name, value);
    }

    @Override
    protected BigDecimal fetch(TgFfiContext context, TgFfiSqlQueryResult qr) {
        return qr.fetchDecimal(context);
    }

    @Override
    protected BigDecimal fetchFor(TgFfiContext context, TgFfiSqlQueryResult qr, Duration timeout) {
        return qr.fetchForDecimal(context, timeout);
    }

    @Override
    protected void assertValueList(List<BigDecimal> expected, List<BigDecimal> actual) {
        assertEquals(expected.size(), actual.size());
        for (int i = 0; i < actual.size(); i++) {
            var e = expected.get(i);
            var a = actual.get(i);

            if (e == null) {
                assertNull(a);
                continue;
            }

            assertEquals(e.setScale(1), a);
        }
    }
}
