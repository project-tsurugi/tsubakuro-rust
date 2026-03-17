import tsurugi_dbapi as tsurugi


def test_constructor():
    value = tsurugi.type_code.Int64()
    assert value.value is None

    value = tsurugi.type_code.Int64(None)
    assert value.value is None

    value = tsurugi.type_code.Int64(123)
    assert value.value == 123
