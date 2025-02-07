package com.tsurugidb.tsubakuro.rust.java.type;

import java.time.Duration;
import java.time.LocalDate;
import java.util.ArrayList;
import java.util.List;

import com.tsurugidb.iceaxe.sql.parameter.TgBindParameter;
import com.tsurugidb.iceaxe.sql.parameter.TgBindVariable;
import com.tsurugidb.iceaxe.sql.result.TsurugiResultEntity;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.service.sql.TgFfiAtomType;
import com.tsurugidb.tsubakuro.rust.java.service.sql.TgFfiSqlQueryResult;
import com.tsurugidb.tsubakuro.rust.java.service.sql.prepare.TgFfiSqlParameter;

class TgFfiTypeDateTest extends TgFfiTypeTester<LocalDate> {

    @Override
    protected String sqlType() {
        return "date";
    }

    @Override
    protected List<LocalDate> values() {
        var list = new ArrayList<LocalDate>();
        list.add(LocalDate.now());
        list.add(LocalDate.of(1970, 1, 1));
        list.add(LocalDate.of(-1, 1, 1));
        list.add(LocalDate.of(0, 1, 1));
        list.add(LocalDate.of(1, 1, 1));
        list.add(LocalDate.of(9999, 12, 31));
        list.add(null);
        return list;
    }

    @Override
    protected TgBindVariable<LocalDate> bindVariable(String name) {
        return TgBindVariable.ofDate(name);
    }

    @Override
    protected TgBindParameter bindParameter(String name, LocalDate value) {
        return TgBindParameter.of(name, value);
    }

    @Override
    protected LocalDate get(TsurugiResultEntity entity, String name) {
        return entity.getDate(name);
    }

    @Override
    protected TgFfiAtomType ffiAtomType() {
        return TgFfiAtomType.DATE;
    }

    @Override
    protected TgFfiSqlParameter ffiParameter(TgFfiContext context, String name, LocalDate value) {
        return TgFfiSqlParameter.ofDate(context, name, value);
    }

    @Override
    protected LocalDate fetch(TgFfiContext context, TgFfiSqlQueryResult qr) {
        return qr.fetchDate(context);
    }

    @Override
    protected LocalDate fetchFor(TgFfiContext context, TgFfiSqlQueryResult qr, Duration timeout) {
        return qr.fetchForDate(context, timeout);
    }
}
