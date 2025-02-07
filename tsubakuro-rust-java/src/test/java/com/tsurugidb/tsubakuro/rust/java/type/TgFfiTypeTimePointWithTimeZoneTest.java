package com.tsurugidb.tsubakuro.rust.java.type;

import static org.junit.jupiter.api.Assertions.assertEquals;

import java.time.Duration;
import java.time.OffsetDateTime;
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

class TgFfiTypeTimePointWithTimeZoneTest extends TgFfiTypeTester<OffsetDateTime> {

    @Override
    protected String sqlType() {
        return "timestamp with time zone";
    }

    @Override
    protected List<OffsetDateTime> values() {
        var list = new ArrayList<OffsetDateTime>();
        list.add(OffsetDateTime.now());
        for (var offset : List.of(ZoneOffset.UTC, ZoneOffset.ofHours(9))) {
            list.add(OffsetDateTime.of(1969, 12, 31, 23, 59, 59, 999_999_999, offset));
            list.add(OffsetDateTime.of(1970, 1, 1, 0, 0, 0, 0, offset));
            list.add(OffsetDateTime.of(2025, 2, 7, 12, 30, 59, 123456789, offset));
            list.add(OffsetDateTime.of(9999, 12, 31, 23, 59, 59, 999_999_999, offset));
            list.add(OffsetDateTime.of(-1, 1, 1, 0, 0, 0, 0, offset));
            list.add(OffsetDateTime.of(0, 1, 1, 0, 0, 0, 0, offset));
        }
        list.add(null);
        return list;
    }

    @Override
    protected TgBindVariable<OffsetDateTime> bindVariable(String name) {
        return TgBindVariable.ofOffsetDateTime(name);
    }

    @Override
    protected TgBindParameter bindParameter(String name, OffsetDateTime value) {
        return TgBindParameter.of(name, value);
    }

    @Override
    protected OffsetDateTime get(TsurugiResultEntity entity, String name) {
        return entity.getOffsetDateTime(name);
    }

    @Override
    protected TgFfiAtomType ffiAtomType() {
        return TgFfiAtomType.TIME_POINT_WITH_TIME_ZONE;
    }

    @Override
    protected TgFfiSqlParameter ffiParameter(TgFfiContext context, String name, OffsetDateTime value) {
        return TgFfiSqlParameter.ofTimePointWithTimeZone(context, name, value);
    }

    @Override
    protected OffsetDateTime fetch(TgFfiContext context, TgFfiSqlQueryResult qr) {
        return qr.fetchTimePointWithTimeZone(context);
    }

    @Override
    protected OffsetDateTime fetchFor(TgFfiContext context, TgFfiSqlQueryResult qr, Duration timeout) {
        return qr.fetchForTimePointWithTimeZone(context, timeout);
    }

    @Override
    protected void assertValueList(List<OffsetDateTime> expected, List<OffsetDateTime> actual) {
        assertEquals(expected.size(), actual.size());
        for (int i = 0; i < actual.size(); i++) {
            var e = toZ(expected.get(i));
            var a = actual.get(i);
            assertEquals(e, a);
        }
    }

    private static OffsetDateTime toZ(OffsetDateTime date) {
        if (date == null) {
            return null;
        }
        return date.withOffsetSameInstant(ZoneOffset.UTC);
    }
}
