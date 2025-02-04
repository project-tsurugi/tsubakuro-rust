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

class TgFfiTypeCharacterTest extends TgFfiTypeTester<String> {

    @Override
    protected String sqlType() {
        return "varchar(10)";
    }

    @Override
    protected List<String> values() {
        var list = new ArrayList<String>();
        list.add("");
        list.add("abc");
        list.add(null);
        return list;
    }

    @Override
    protected TgBindVariable<String> bindVariable(String name) {
        return TgBindVariable.ofString(name);
    }

    @Override
    protected TgBindParameter bindParameter(String name, String value) {
        return TgBindParameter.of(name, value);
    }

    @Override
    protected String get(TsurugiResultEntity entity, String name) {
        return entity.getString(name);
    }

    @Override
    protected TgFfiAtomType ffiAtomType() {
        return TgFfiAtomType.CHARACTER;
    }

    @Override
    protected TgFfiSqlParameter ffiParameter(TgFfiContext context, String name, String value) {
        return TgFfiSqlParameter.ofCharacter(context, name, value);
    }

    @Override
    protected String fetch(TgFfiContext context, TgFfiSqlQueryResult qr) {
        return qr.fetchCharacter(context);
    }

    @Override
    protected String fetchFor(TgFfiContext context, TgFfiSqlQueryResult qr, Duration timeout) {
        return qr.fetchForCharacter(context, timeout);
    }
}
