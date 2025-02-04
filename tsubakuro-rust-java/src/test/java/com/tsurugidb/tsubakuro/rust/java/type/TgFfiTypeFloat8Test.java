package com.tsurugidb.tsubakuro.rust.java.type;

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

class TgFfiTypeFloat8Test extends TgFfiTypeTester<Double> {

    @Override
    protected String sqlType() {
        return "double";
    }

    @Override
    protected List<Double> values() {
        var list = new ArrayList<Double>();
        list.add(Double.MIN_VALUE);
        list.add(-1d);
        list.add(0d);
        list.add(1d);
        list.add(123.4d);
        list.add(Double.MAX_VALUE);
        list.add(null);
        list.add(Double.NEGATIVE_INFINITY);
        list.add(Double.POSITIVE_INFINITY);
        list.add(Double.NaN);
        return list;
    }

    @Override
    protected TgBindVariable<Double> bindVariable(String name) {
        return TgBindVariable.ofDouble(name);
    }

    @Override
    protected TgBindParameter bindParameter(String name, Double value) {
        return TgBindParameter.of(name, value);
    }

    @Override
    protected Double get(TsurugiResultEntity entity, String name) {
        return entity.getDouble(name);
    }

    @Override
    protected TgFfiAtomType ffiAtomType() {
        return TgFfiAtomType.FLOAT8;
    }

    @Override
    protected TgFfiSqlParameter ffiParameter(TgFfiContext context, String name, Double value) {
        return TgFfiSqlParameter.ofFloat8(context, name, value);
    }

    @Override
    protected Double fetch(TgFfiContext context, TgFfiSqlQueryResult qr) {
        return qr.fetchFloat8(context);
    }

    @Override
    protected Double fetchFor(TgFfiContext context, TgFfiSqlQueryResult qr, Duration timeout) {
        return qr.fetchForFloat8(context, timeout);
    }
}
