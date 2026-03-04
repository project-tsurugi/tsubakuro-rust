import tsubakuro_rust_python as tsurugi


def test_error():
    try:
        raise tsurugi.Error("test")
    except tsurugi.Error as e:
        assert str(e) == "test"
