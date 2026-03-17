import tsurugi_dbapi as tsurugi
import datetime

JST = datetime.timezone(datetime.timedelta(hours=9))
UTC = datetime.timezone(datetime.timedelta(hours=0))


def test_constructor():
    value = tsurugi.type_code.OffsetTime()
    assert value.value is None
    assert value.nanosecond is None

    value = tsurugi.type_code.OffsetTime(None)
    assert value.value is None
    assert value.nanosecond is None

    value = tsurugi.type_code.OffsetTime(datetime.time(12, 34, 56, 123456, tzinfo=JST))
    assert value.value == datetime.time(12, 34, 56, 123456, tzinfo=JST)
    assert value.nanosecond == 123456000

    value = tsurugi.type_code.OffsetTime(datetime.time(12, 34, 56, 123456, tzinfo=UTC))
    assert value.value == datetime.time(12, 34, 56, 123456, tzinfo=UTC)
    assert value.nanosecond == 123456000

    value = tsurugi.type_code.OffsetTime(datetime.time(12, 34, 56, 123456))
    assert value.value == datetime.time(12, 34, 56, 123456, tzinfo=UTC)
    assert value.nanosecond == 123456000

    value = tsurugi.type_code.OffsetTime(
        datetime.time(12, 34, 56, tzinfo=JST), 123456789
    )
    assert value.value == datetime.time(12, 34, 56, 123456, tzinfo=JST)
    assert value.nanosecond == 123456789
    assert value.__repr__() == "OffsetTime(12:34:56.123456789 +09:00)"

    value = tsurugi.type_code.OffsetTime(
        datetime.time(12, 34, 56, tzinfo=UTC), 123456789
    )
    assert value.value == datetime.time(12, 34, 56, 123456, tzinfo=UTC)
    assert value.nanosecond == 123456789

    value = tsurugi.type_code.OffsetTime(datetime.time(12, 34, 56), 123456789)
    assert value.value == datetime.time(12, 34, 56, 123456, tzinfo=UTC)
    assert value.nanosecond == 123456789


def test_of():
    value = tsurugi.type_code.OffsetTime.of(12, 34, 56, 123456789, JST)
    assert value.value == datetime.time(12, 34, 56, 123456, tzinfo=JST)
    assert value.nanosecond == 123456789

    value = tsurugi.type_code.OffsetTime.of()
    assert value.value == datetime.time(0, 0, 0, 0, tzinfo=UTC)
    assert value.nanosecond == 0


def test_raw():
    value = tsurugi.type_code.OffsetTime.raw(
        ((12 * 60 + 34) * 60 + 56) * 1_000_000_000 + 123456789, 9 * 60
    )
    assert value.value == datetime.time(12, 34, 56, 123456, tzinfo=JST)
    assert value.nanosecond == 123456789
