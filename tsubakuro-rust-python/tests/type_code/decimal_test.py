import tsubakuro_rust_python as tsurugi
import decimal


def test_constructor():
    value = tsurugi.type_code.Decimal()
    assert value.value is None

    value = tsurugi.type_code.Decimal(None)
    assert value.value is None

    value = tsurugi.type_code.Decimal(123)
    assert value.value == decimal.Decimal(123)

    value = tsurugi.type_code.Decimal(123.5)
    assert value.value == decimal.Decimal(123.5)

    value = tsurugi.type_code.Decimal(decimal.Decimal("123.4"))
    assert value.value == decimal.Decimal("123.4")

    value = tsurugi.type_code.Decimal("123.4")
    assert value.value == decimal.Decimal("123.4")


def test_raw():
    value = tsurugi.type_code.Decimal.raw([0x04, 0xD2], -1)
    assert value.value == decimal.Decimal("123.4")
