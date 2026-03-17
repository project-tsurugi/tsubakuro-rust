import tsurugi_dbapi as tsurugi


def test_constructor():
    value = tsurugi.type_code.Float64()
    assert value.value is None

    value = tsurugi.type_code.Float64(None)
    assert value.value is None

    value = tsurugi.type_code.Float64(123.5)
    assert value.value == 123.5
