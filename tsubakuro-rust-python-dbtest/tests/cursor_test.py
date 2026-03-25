import pytest
import tsurugi_dbapi as tsurugi


def test_connection(connection, endpoint):
    with connection.cursor() as cursor:
        c = cursor.connection
        assert c is connection
        assert c == connection
        assert hash(c) == hash(connection)

        with tsurugi.connect(
            endpoint=endpoint, user="tsurugi", password="password"
        ) as new_connection:
            assert new_connection is not c
            assert new_connection != c

        assert c.closed is False
        assert connection.closed is False
        c.close()
        assert c.closed is True
        assert connection.closed is True


def test_connection_drop(connection):
    with connection.cursor() as cursor:
        cursor.execute("drop table if exists tsubakuro_rust_python_test")
        cursor.execute(
            "create table tsubakuro_rust_python_test (foo int primary key, bar int, zzz varchar(10))"
        )

        c = cursor.connection
        c.commit()
        del c

        # Verify that the cursor continues to function correctly even after cursor.connection is dropped
        cursor.execute("insert into tsubakuro_rust_python_test values (1, 100, 'abc')")
        connection.commit()


def test_with_exception(connection):
    with pytest.raises(Exception, match="Test exception in with block"):
        with connection.cursor():
            raise Exception("Test exception in with block")
