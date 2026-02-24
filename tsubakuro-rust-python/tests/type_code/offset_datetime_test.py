import tsubakuro_rust_python as tsurugi
import datetime

JST = datetime.timezone(datetime.timedelta(hours=9))
UTC = datetime.timezone(datetime.timedelta(hours=0))


def test_constructor():
    value = tsurugi.type_code.OffsetDatetime()
    assert value.value is None
    assert value.nanosecond is None

    value = tsurugi.type_code.OffsetDatetime(None)
    assert value.value is None
    assert value.nanosecond is None

    value = tsurugi.type_code.OffsetDatetime(
        datetime.datetime(2026, 1, 27, 16, 24, 30, 123456, tzinfo=JST)
    )
    assert value.value == datetime.datetime(2026, 1, 27, 16, 24, 30, 123456, tzinfo=JST)
    assert value.nanosecond == 123456000

    value = tsurugi.type_code.OffsetDatetime(
        datetime.datetime(2026, 1, 27, 16, 24, 30, 123456, tzinfo=UTC)
    )
    assert value.value == datetime.datetime(2026, 1, 27, 16, 24, 30, 123456, tzinfo=UTC)
    assert value.nanosecond == 123456000

    value = tsurugi.type_code.OffsetDatetime(
        datetime.datetime(2026, 1, 27, 16, 24, 30, tzinfo=JST), 123456789
    )
    assert value.value == datetime.datetime(2026, 1, 27, 16, 24, 30, 123456, tzinfo=JST)
    assert value.nanosecond == 123456789
    assert value.__repr__() == "OffsetDatetime(2026-01-27 16:24:30.123456789 +09:00)"

    value = tsurugi.type_code.OffsetDatetime(
        datetime.datetime(2026, 1, 27, 16, 24, 30, tzinfo=UTC), 123456789
    )
    assert value.value == datetime.datetime(2026, 1, 27, 16, 24, 30, 123456, tzinfo=UTC)
    assert value.nanosecond == 123456789


def test_of():
    value = tsurugi.type_code.OffsetDatetime.of(2026, 1, 27, 16, 24, 30, 123456789, JST)
    assert value.value == datetime.datetime(2026, 1, 27, 16, 24, 30, 123456, tzinfo=JST)
    assert value.nanosecond == 123456789

    value = tsurugi.type_code.OffsetDatetime.of(2026, 1, 27)
    assert value.value == datetime.datetime(2026, 1, 27, 0, 0, 0, 0, tzinfo=UTC)
    assert value.nanosecond == 0


def test_raw():
    value = tsurugi.type_code.OffsetDatetime.raw(1769531070, 123456789, 9 * 60)
    assert value.value == datetime.datetime(2026, 1, 27, 16, 24, 30, 123456, tzinfo=JST)
    assert value.nanosecond == 123456789
