import pytest
import tsurugi_dbapi as tsurugi


def test_execute_after_commit_error(connection):
    with connection.cursor() as cursor:
        cursor.execute("drop table if exists tsubakuro_rust_python_test")
        cursor.execute(
            "create table tsubakuro_rust_python_test (foo int primary key, bar int, zzz varchar(10))"
        )
        connection.commit()

        cursor.execute("insert into tsubakuro_rust_python_test values (1, 100, 'abc')")
        connection.commit()

        with pytest.raises(tsurugi.error.UniqueConstraintViolationException):
            cursor.execute(
                "insert into tsubakuro_rust_python_test values (1, 100, 'abc')"
            )

        with pytest.raises(tsurugi.error.InactiveTransactionException):
            connection.commit()

        cursor.execute("select * from tsubakuro_rust_python_test order by foo")
        cursor.fetchall()
        connection.commit()


def test_rollback_after_error(connection):
    with connection.cursor() as cursor:
        cursor.execute("drop table if exists tsubakuro_rust_python_test")
        cursor.execute(
            "create table tsubakuro_rust_python_test (foo int primary key, bar int, zzz varchar(10))"
        )
        connection.commit()

        cursor.execute("insert into tsubakuro_rust_python_test values (1, 100, 'abc')")
        connection.commit()

        with pytest.raises(tsurugi.error.UniqueConstraintViolationException):
            cursor.execute(
                "insert into tsubakuro_rust_python_test values (1, 100, 'abc')"
            )

        connection.rollback()

        cursor.execute("select * from tsubakuro_rust_python_test order by foo")
        cursor.fetchall()
        connection.commit()
