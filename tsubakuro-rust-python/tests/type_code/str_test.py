import tsubakuro_rust_python as tsurugi


def test_constructor():
    value = tsurugi.type_code.Str()
    assert value.value is None

    value = tsurugi.type_code.Str(None)
    assert value.value is None

    value = tsurugi.type_code.Str("abc")
    assert value.value == "abc"

    value = tsurugi.type_code.Str("")
    assert value.value == ""
