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

class TgFfiTypeInt8Test extends TgFfiTypeTester<Long> {

	@Override
	protected String sqlType() {
		return "bigint";
	}

	@Override
	protected List<Long> values() {
		var list = new ArrayList<Long>();
		list.add(Long.MIN_VALUE);
		list.add(-1L);
		list.add(0L);
		list.add(1L);
		list.add(123L);
		list.add(Long.MAX_VALUE);
		list.add(null);
		return list;
	}

	@Override
	protected TgBindVariable<Long> bindVariable(String name) {
		return TgBindVariable.ofLong(name);
	}

	@Override
	protected TgBindParameter bindParameter(String name, Long value) {
		return TgBindParameter.of(name, value);
	}

	@Override
	protected Long get(TsurugiResultEntity entity, String name) {
		return entity.getLong(name);
	}

	@Override
	protected TgFfiAtomType ffiAtomType() {
		return TgFfiAtomType.INT8;
	}

	@Override
	protected TgFfiSqlParameter ffiParameter(TgFfiContext context, String name, Long value) {
		return TgFfiSqlParameter.ofInt8(context, name, value);
	}

	@Override
	protected Long fetch(TgFfiContext context, TgFfiSqlQueryResult qr) {
		return qr.fetchInt8(context);
	}

	@Override
	protected Long fetchFor(TgFfiContext context, TgFfiSqlQueryResult qr, Duration timeout) {
		return qr.fetchForInt8(context, timeout);
	}
}
