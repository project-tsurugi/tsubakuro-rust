import tsurugi_dbapi as tsurugi
import datetime


def test_constructor():
    value = tsurugi.type_code.Date()
    assert value.value is None

    value = tsurugi.type_code.Date(None)
    assert value.value is None

    value = tsurugi.type_code.Date(datetime.date(2026, 1, 27))
    assert value.value == datetime.date(2026, 1, 27)


def test_of():
    value = tsurugi.type_code.Date.of(2026, 1, 27)
    assert value.value == datetime.date(2026, 1, 27)


def test_raw():
    value = tsurugi.type_code.Date.raw(20480)
    assert value.value == datetime.date(2026, 1, 27)
