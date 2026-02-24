import tsubakuro_rust_python as tsurugi


def test_constructor():
    value = tsurugi.type_code.Bytes()
    assert value.value is None

    value = tsurugi.type_code.Bytes(None)
    assert value.value is None

    value = tsurugi.type_code.Bytes(b"\x12\x34\x56")
    assert value.value == b"\x12\x34\x56"

    value = tsurugi.type_code.Bytes(b"")
    assert value.value == b""
