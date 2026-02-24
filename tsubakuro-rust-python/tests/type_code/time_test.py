import tsubakuro_rust_python as tsurugi
import datetime


def test_constructor():
    value = tsurugi.type_code.Time()
    assert value.value is None
    assert value.nanosecond is None

    value = tsurugi.type_code.Time(None)
    assert value.value is None
    assert value.nanosecond is None

    value = tsurugi.type_code.Time(datetime.time(12, 34, 56, 123456))
    assert value.value == datetime.time(12, 34, 56, 123456)
    assert value.nanosecond == 123456000

    value = tsurugi.type_code.Time(datetime.time(12, 34, 56), 123456789)
    assert value.value == datetime.time(12, 34, 56, 123456)
    assert value.nanosecond == 123456789
    assert value.__repr__() == "Time(12:34:56.123456789)"


def test_of():
    value = tsurugi.type_code.Time.of(12, 34, 56, 123456789)
    assert value.value == datetime.time(12, 34, 56, 123456)
    assert value.nanosecond == 123456789

    value = tsurugi.type_code.Time.of()
    assert value.value == datetime.time(0, 0, 0, 0)
    assert value.nanosecond == 0


def test_raw():
    value = tsurugi.type_code.Time.raw(
        ((12 * 60 + 34) * 60 + 56) * 1_000_000_000 + 123456789
    )
    assert value.value == datetime.time(12, 34, 56, 123456)
    assert value.nanosecond == 123456789
