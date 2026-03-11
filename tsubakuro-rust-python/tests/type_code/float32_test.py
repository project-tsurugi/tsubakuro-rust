import tsubakuro_rust_python as tsurugi


def test_constructor():
    value = tsurugi.type_code.Float32()
    assert value.value is None

    value = tsurugi.type_code.Float32(None)
    assert value.value is None

    value = tsurugi.type_code.Float32(123.5)
    assert value.value == 123.5
