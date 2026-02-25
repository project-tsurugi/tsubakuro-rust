import tsubakuro_rust_python as tsurugi


def test_connection(connection, endpoint):
    with connection.cursor() as cursor:
        c = cursor.connection
        assert c is not connection
        assert c == connection
        assert c.__hash__() == connection.__hash__()

        with tsurugi.connect(
            endpoint=endpoint, user="tsurugi", password="password"
        ) as new_connection:
            assert new_connection != c
            assert new_connection.__hash__() != c.__hash__()

        assert c.closed is False
        assert connection.closed is False
        c.close()
        assert c.closed is True
        assert connection.closed is True
