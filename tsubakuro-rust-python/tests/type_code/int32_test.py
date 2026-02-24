import tsubakuro_rust_python as tsurugi


def test_constructor():
    value = tsurugi.type_code.Int32()
    assert value.value is None

    value = tsurugi.type_code.Int32(None)
    assert value.value is None

    value = tsurugi.type_code.Int32(123)
    assert value.value == 123
