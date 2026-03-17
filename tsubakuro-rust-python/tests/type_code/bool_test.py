import tsurugi_dbapi as tsurugi


def test_constructor():
    value = tsurugi.type_code.Bool()
    assert value.value is None

    value = tsurugi.type_code.Bool(None)
    assert value.value is None

    value = tsurugi.type_code.Bool(True)
    assert value.value is True

    value = tsurugi.type_code.Bool(False)
    assert value.value is False
