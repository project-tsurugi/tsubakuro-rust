import tsurugi_dbapi as tsurugi


def test_error():
    try:
        raise tsurugi.Error("test")
    except tsurugi.Error as e:
        assert str(e) == "test"
