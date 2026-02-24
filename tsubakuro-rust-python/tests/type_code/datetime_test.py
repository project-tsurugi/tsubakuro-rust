import tsubakuro_rust_python as tsurugi
import datetime


def test_constructor():
    value = tsurugi.type_code.Datetime()
    assert value.value is None
    assert value.nanosecond is None

    value = tsurugi.type_code.Datetime(None)
    assert value.value is None
    assert value.nanosecond is None

    value = tsurugi.type_code.Datetime(
        datetime.datetime(2026, 1, 27, 16, 24, 30, 123456)
    )
    assert value.value == datetime.datetime(2026, 1, 27, 16, 24, 30, 123456)
    assert value.nanosecond == 123456000

    value = tsurugi.type_code.Datetime(
        datetime.datetime(2026, 1, 27, 16, 24, 30), 123456789
    )
    assert value.value == datetime.datetime(2026, 1, 27, 16, 24, 30, 123456)
    assert value.nanosecond == 123456789
    assert value.__repr__() == "Datetime(2026-01-27 16:24:30.123456789)"


def test_of():
    value = tsurugi.type_code.Datetime.of(2026, 1, 27, 16, 24, 30, 123456789)
    assert value.value == datetime.datetime(2026, 1, 27, 16, 24, 30, 123456)
    assert value.nanosecond == 123456789

    value = tsurugi.type_code.Datetime.of(2026, 1, 27)
    assert value.value == datetime.datetime(2026, 1, 27, 0, 0, 0, 0)
    assert value.nanosecond == 0


def test_raw():
    value = tsurugi.type_code.Datetime.raw(1769531070, 123456789)
    assert value.value == datetime.datetime(2026, 1, 27, 16, 24, 30, 123456)
    assert value.nanosecond == 123456789
