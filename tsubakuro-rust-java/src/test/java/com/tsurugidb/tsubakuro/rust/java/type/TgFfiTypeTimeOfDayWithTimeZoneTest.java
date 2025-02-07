package com.tsurugidb.tsubakuro.rust.java.type;

import static org.junit.jupiter.api.Assertions.assertEquals;

import java.time.Duration;
import java.time.OffsetTime;
import java.time.ZoneOffset;
import java.util.ArrayList;
import java.util.List;

import com.tsurugidb.iceaxe.sql.parameter.TgBindParameter;
import com.tsurugidb.iceaxe.sql.parameter.TgBindVariable;
import com.tsurugidb.iceaxe.sql.result.TsurugiResultEntity;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.service.sql.TgFfiAtomType;
import com.tsurugidb.tsubakuro.rust.java.service.sql.TgFfiSqlQueryResult;
import com.tsurugidb.tsubakuro.rust.java.service.sql.prepare.TgFfiSqlParameter;

class TgFfiTypeTimeOfDayWithTimeZoneTest extends TgFfiTypeTester<OffsetTime> {

    @Override
    protected String sqlType() {
        return "time with time zone";
    }

    @Override
    protected List<OffsetTime> values() {
        var offset = ZoneOffset.ofHours(9);

        var list = new ArrayList<OffsetTime>();
        list.add(OffsetTime.now());
        list.add(OffsetTime.of(0, 0, 0, 0, ZoneOffset.UTC));
        list.add(OffsetTime.of(1, 30, 59, 123456789, ZoneOffset.UTC));
        list.add(OffsetTime.of(23, 59, 59, 999_999_999, ZoneOffset.UTC));
        list.add(OffsetTime.of(0, 0, 0, 0, offset));
        list.add(OffsetTime.of(1, 30, 59, 123456789, offset));
        list.add(OffsetTime.of(23, 59, 59, 999_999_999, offset));
        list.add(null);
        return list;
    }

    @Override
    protected TgBindVariable<OffsetTime> bindVariable(String name) {
        return TgBindVariable.ofOffsetTime(name);
    }

    @Override
    protected TgBindParameter bindParameter(String name, OffsetTime value) {
        return TgBindParameter.of(name, value);
    }

    @Override
    protected OffsetTime get(TsurugiResultEntity entity, String name) {
        return entity.getOffsetTime(name);
    }

    @Override
    protected TgFfiAtomType ffiAtomType() {
        return TgFfiAtomType.TIME_OF_DAY_WITH_TIME_ZONE;
    }

    @Override
    protected TgFfiSqlParameter ffiParameter(TgFfiContext context, String name, OffsetTime value) {
        return TgFfiSqlParameter.ofTimeOfDayWithTimeZone(context, name, value);
    }

    @Override
    protected OffsetTime fetch(TgFfiContext context, TgFfiSqlQueryResult qr) {
        return qr.fetchTimeOfDayWithTimeZone(context);
    }

    @Override
    protected OffsetTime fetchFor(TgFfiContext context, TgFfiSqlQueryResult qr, Duration timeout) {
        return qr.fetchForTimeOfDayWithTimeZone(context, timeout);
    }

    @Override
    protected void assertValueList(List<OffsetTime> expected, List<OffsetTime> actual) {
        assertEquals(expected.size(), actual.size());
        for (int i = 0; i < actual.size(); i++) {
            var e = toZ(expected.get(i));
            var a = actual.get(i);
            assertEquals(e, a);
        }
    }

    private static OffsetTime toZ(OffsetTime date) {
        if (date == null) {
            return null;
        }
        return date.withOffsetSameInstant(ZoneOffset.UTC);
    }
}
