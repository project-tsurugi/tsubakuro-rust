package com.tsurugidb.tsubakuro.rust.java.type;

import java.time.Duration;
import java.time.LocalDateTime;
import java.util.ArrayList;
import java.util.List;

import com.tsurugidb.iceaxe.sql.parameter.TgBindParameter;
import com.tsurugidb.iceaxe.sql.parameter.TgBindVariable;
import com.tsurugidb.iceaxe.sql.result.TsurugiResultEntity;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.service.sql.TgFfiAtomType;
import com.tsurugidb.tsubakuro.rust.java.service.sql.TgFfiSqlQueryResult;
import com.tsurugidb.tsubakuro.rust.java.service.sql.prepare.TgFfiSqlParameter;

class TgFfiTypeTimePointTest extends TgFfiTypeTester<LocalDateTime> {

    @Override
    protected String sqlType() {
        return "timestamp";
    }

    @Override
    protected List<LocalDateTime> values() {
        var list = new ArrayList<LocalDateTime>();
        list.add(LocalDateTime.now());
        list.add(LocalDateTime.of(1969, 12, 31, 23, 59, 59, 999_999_999));
        list.add(LocalDateTime.of(1970, 1, 1, 0, 0, 0));
        list.add(LocalDateTime.of(2025, 2, 7, 12, 30, 59, 123456789));
        list.add(LocalDateTime.of(9999, 12, 31, 23, 59, 59, 999_999_999));
        list.add(LocalDateTime.of(-1, 1, 1, 0, 0, 0));
        list.add(LocalDateTime.of(0, 1, 1, 0, 0, 0));
        list.add(null);
        return list;
    }

    @Override
    protected TgBindVariable<LocalDateTime> bindVariable(String name) {
        return TgBindVariable.ofDateTime(name);
    }

    @Override
    protected TgBindParameter bindParameter(String name, LocalDateTime value) {
        return TgBindParameter.of(name, value);
    }

    @Override
    protected LocalDateTime get(TsurugiResultEntity entity, String name) {
        return entity.getDateTime(name);
    }

    @Override
    protected TgFfiAtomType ffiAtomType() {
        return TgFfiAtomType.TIME_POINT;
    }

    @Override
    protected TgFfiSqlParameter ffiParameter(TgFfiContext context, String name, LocalDateTime value) {
        return TgFfiSqlParameter.ofTimePoint(context, name, value);
    }

    @Override
    protected LocalDateTime fetch(TgFfiContext context, TgFfiSqlQueryResult qr) {
        return qr.fetchTimePoint(context);
    }

    @Override
    protected LocalDateTime fetchFor(TgFfiContext context, TgFfiSqlQueryResult qr, Duration timeout) {
        return qr.fetchForTimePoint(context, timeout);
    }
}
