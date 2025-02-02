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

class TgFfiTypeInt4Test extends TgFfiTypeTester<Integer> {

	@Override
	protected String sqlType() {
		return "int";
	}

	@Override
	protected List<Integer> values() {
		var list = new ArrayList<Integer>();
		list.add(Integer.MIN_VALUE);
		list.add(-1);
		list.add(0);
		list.add(1);
		list.add(123);
		list.add(Integer.MAX_VALUE);
		list.add(null);
		return list;
	}

	@Override
	protected TgBindVariable<Integer> bindVariable(String name) {
		return TgBindVariable.ofInt(name);
	}

	@Override
	protected TgBindParameter bindParameter(String name, Integer value) {
		return TgBindParameter.of(name, value);
	}

	@Override
	protected Integer get(TsurugiResultEntity entity, String name) {
		return entity.getInt(name);
	}

	@Override
	protected TgFfiAtomType ffiAtomType() {
		return TgFfiAtomType.INT4;
	}

	@Override
	protected TgFfiSqlParameter ffiParameter(TgFfiContext context, String name, Integer value) {
		return TgFfiSqlParameter.ofInt4(context, name, value);
	}

	@Override
	protected Integer fetch(TgFfiContext context, TgFfiSqlQueryResult qr) {
		return qr.fetchInt4(context);
	}

	@Override
	protected Integer fetchFor(TgFfiContext context, TgFfiSqlQueryResult qr, Duration timeout) {
		return qr.fetchForInt4(context, timeout);
	}
}
