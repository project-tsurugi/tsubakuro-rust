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

class TgFfiTypeFloat4Test extends TgFfiTypeTester<Float> {

    @Override
    protected String sqlType() {
        return "real";
    }

    @Override
    protected List<Float> values() {
        var list = new ArrayList<Float>();
        list.add(Float.MIN_VALUE);
        list.add(-1f);
        list.add(0f);
        list.add(1f);
        list.add(123.4f);
        list.add(Float.MAX_VALUE);
        list.add(null);
        list.add(Float.NEGATIVE_INFINITY);
        list.add(Float.POSITIVE_INFINITY);
        list.add(Float.NaN);
        return list;
    }

    @Override
    protected TgBindVariable<Float> bindVariable(String name) {
        return TgBindVariable.ofFloat(name);
    }

    @Override
    protected TgBindParameter bindParameter(String name, Float value) {
        return TgBindParameter.of(name, value);
    }

    @Override
    protected Float get(TsurugiResultEntity entity, String name) {
        return entity.getFloat(name);
    }

    @Override
    protected TgFfiAtomType ffiAtomType() {
        return TgFfiAtomType.FLOAT4;
    }

    @Override
    protected TgFfiSqlParameter ffiParameter(TgFfiContext context, String name, Float value) {
        return TgFfiSqlParameter.ofFloat4(context, name, value);
    }

    @Override
    protected Float fetch(TgFfiContext context, TgFfiSqlQueryResult qr) {
        return qr.fetchFloat4(context);
    }

    @Override
    protected Float fetchFor(TgFfiContext context, TgFfiSqlQueryResult qr, Duration timeout) {
        return qr.fetchForFloat4(context, timeout);
    }
}
