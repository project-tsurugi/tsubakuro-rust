package com.tsurugidb.tsubakuro.rust.java.type;

import java.time.Duration;
import java.time.LocalTime;
import java.util.ArrayList;
import java.util.List;

import com.tsurugidb.iceaxe.sql.parameter.TgBindParameter;
import com.tsurugidb.iceaxe.sql.parameter.TgBindVariable;
import com.tsurugidb.iceaxe.sql.result.TsurugiResultEntity;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.service.sql.TgFfiAtomType;
import com.tsurugidb.tsubakuro.rust.java.service.sql.TgFfiSqlQueryResult;
import com.tsurugidb.tsubakuro.rust.java.service.sql.prepare.TgFfiSqlParameter;

class TgFfiTypeTimeOfDayTest extends TgFfiTypeTester<LocalTime> {

    @Override
    protected String sqlType() {
        return "time";
    }

    @Override
    protected List<LocalTime> values() {
        var list = new ArrayList<LocalTime>();
        list.add(LocalTime.now());
        list.add(LocalTime.of(0, 0, 0));
        list.add(LocalTime.of(12, 30, 59, 123456789));
        list.add(LocalTime.of(23, 59, 59, 999_999_999));
        list.add(null);
        return list;
    }

    @Override
    protected TgBindVariable<LocalTime> bindVariable(String name) {
        return TgBindVariable.ofTime(name);
    }

    @Override
    protected TgBindParameter bindParameter(String name, LocalTime value) {
        return TgBindParameter.of(name, value);
    }

    @Override
    protected LocalTime get(TsurugiResultEntity entity, String name) {
        return entity.getTime(name);
    }

    @Override
    protected TgFfiAtomType ffiAtomType() {
        return TgFfiAtomType.TIME_OF_DAY;
    }

    @Override
    protected TgFfiSqlParameter ffiParameter(TgFfiContext context, String name, LocalTime value) {
        return TgFfiSqlParameter.ofTimeOfDay(context, name, value);
    }

    @Override
    protected LocalTime fetch(TgFfiContext context, TgFfiSqlQueryResult qr) {
        return qr.fetchTimeOfDay(context);
    }

    @Override
    protected LocalTime fetchFor(TgFfiContext context, TgFfiSqlQueryResult qr, Duration timeout) {
        return qr.fetchForTimeOfDay(context, timeout);
    }
}
