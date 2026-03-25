import tsurugi_dbapi as tsurugi
from tsurugi_dbapi import ProgrammingError


def drop_and_create_table(connection):
    with connection.cursor() as cursor:
        cursor.execute("drop table if exists tsubakuro_rust_python_test")
        cursor.execute(
            "create table tsubakuro_rust_python_test ("
            "  foo int primary key,"
            "  bar bigint,"
            "  zzz varchar(10)"
            ")"
        )
        connection.commit()

        cursor.execute(
            "insert into tsubakuro_rust_python_test values"
            " (1, 100, 'abc'),"
            " (2, 200, 'def'),"
            " (3, 300, 'ghi')"
        )
        assert cursor.rowcount == 3
        connection.commit()


def test_description(connection):
    drop_and_create_table(connection)
    with connection.cursor() as cursor:
        cursor.execute("select * from tsubakuro_rust_python_test order by foo")
        desc = cursor.description
        assert desc == (
            ("foo", "Int32", None, None, None, None, None),
            ("bar", "Int64", None, None, None, None, None),
            ("zzz", "Str", None, 10, None, None, None),
        )

        cursor.clear()  # close QueryResult in cursor
        connection.commit()


def test_fetchone(connection):
    drop_and_create_table(connection)
    with connection.cursor() as cursor:
        cursor.execute("select * from tsubakuro_rust_python_test order by foo")
        assert cursor.rownumber == 0
        row = cursor.fetchone()
        assert cursor.rownumber == 1
        assert row == (1, 100, "abc")
        row = cursor.fetchone()
        assert cursor.rownumber == 2
        assert row == (2, 200, "def")
        row = cursor.fetchone()
        assert cursor.rownumber == 3
        assert row == (3, 300, "ghi")
        row = cursor.fetchone()
        assert cursor.rownumber == 3
        assert row is None
        connection.commit()

        cursor.execute("select * from tsubakuro_rust_python_test order by foo")
        assert cursor.rownumber == 0
        row = cursor.fetchone()
        assert cursor.rownumber == 1
        assert row == (1, 100, "abc")
        cursor.close()
        connection.commit()


def test_arraysize(connection):
    with connection.cursor() as cursor:
        assert cursor.arraysize == 1

        cursor.arraysize = 10
        assert cursor.arraysize == 10

        cursor.arraysize = 0
        assert cursor.arraysize == 1

        cursor.arraysize = -1
        assert cursor.arraysize == 1


def test_fetchmany_arraysize(connection):
    drop_and_create_table(connection)

    with connection.cursor() as cursor:
        assert cursor.arraysize == 1

        cursor.execute("select * from tsubakuro_rust_python_test order by foo")
        assert cursor.rownumber == 0
        rows = cursor.fetchmany()
        assert cursor.rownumber == 1
        assert rows == [(1, 100, "abc")]
        rows = cursor.fetchmany()
        assert cursor.rownumber == 2
        assert rows == [(2, 200, "def")]
        rows = cursor.fetchmany()
        assert cursor.rownumber == 3
        assert rows == [(3, 300, "ghi")]
        rows = cursor.fetchmany()
        assert cursor.rownumber == 3
        assert rows == []
        connection.commit()

        cursor.arraysize = 2
        assert cursor.arraysize == 2

        cursor.execute("select * from tsubakuro_rust_python_test order by foo")
        assert cursor.rownumber == 0
        rows = cursor.fetchmany()
        assert cursor.rownumber == 2
        assert rows == [(1, 100, "abc"), (2, 200, "def")]
        rows = cursor.fetchmany()
        assert cursor.rownumber == 3
        assert rows == [(3, 300, "ghi")]
        rows = cursor.fetchmany()
        assert cursor.rownumber == 3
        assert rows == []
        connection.commit()

        cursor.arraysize = 3
        assert cursor.arraysize == 3

        cursor.execute("select * from tsubakuro_rust_python_test order by foo")
        assert cursor.rownumber == 0
        rows = cursor.fetchmany()
        assert cursor.rownumber == 3
        assert rows == [(1, 100, "abc"), (2, 200, "def"), (3, 300, "ghi")]
        rows = cursor.fetchmany()
        assert cursor.rownumber == 3
        assert rows == []
        connection.commit()

        cursor.arraysize = 4
        assert cursor.arraysize == 4

        cursor.execute("select * from tsubakuro_rust_python_test order by foo")
        assert cursor.rownumber == 0
        rows = cursor.fetchmany()
        assert cursor.rownumber == 3
        assert rows == [(1, 100, "abc"), (2, 200, "def"), (3, 300, "ghi")]
        rows = cursor.fetchmany()
        assert cursor.rownumber == 3
        assert rows == []
        connection.commit()


def test_fetchmany_arg(connection):
    drop_and_create_table(connection)

    with connection.cursor() as cursor:
        assert cursor.arraysize == 1

        cursor.execute("select * from tsubakuro_rust_python_test order by foo")
        assert cursor.rownumber == 0
        rows = cursor.fetchmany(2)
        assert cursor.rownumber == 2
        assert rows == [(1, 100, "abc"), (2, 200, "def")]
        rows = cursor.fetchmany(2)
        assert cursor.rownumber == 3
        assert rows == [(3, 300, "ghi")]
        rows = cursor.fetchmany()
        assert cursor.rownumber == 3
        assert rows == []
        connection.commit()

        assert cursor.arraysize == 1

        cursor.execute("select * from tsubakuro_rust_python_test order by foo")
        assert cursor.rownumber == 0
        rows = cursor.fetchmany(4)
        assert cursor.rownumber == 3
        assert rows == [(1, 100, "abc"), (2, 200, "def"), (3, 300, "ghi")]
        rows = cursor.fetchmany(4)
        assert cursor.rownumber == 3
        assert rows == []
        connection.commit()


def test_fetchmany_size(connection):
    drop_and_create_table(connection)

    with connection.cursor() as cursor:
        assert cursor.arraysize == 1

        cursor.execute("select * from tsubakuro_rust_python_test order by foo")
        rows = cursor.fetchmany(size=2)
        assert rows == [(1, 100, "abc"), (2, 200, "def")]
        rows = cursor.fetchmany(size=2)
        assert rows == [(3, 300, "ghi")]
        rows = cursor.fetchmany()
        assert rows == []
        connection.commit()

        assert cursor.arraysize == 1

        cursor.execute("select * from tsubakuro_rust_python_test order by foo")
        rows = cursor.fetchmany(size=4)
        assert rows == [(1, 100, "abc"), (2, 200, "def"), (3, 300, "ghi")]
        rows = cursor.fetchmany()
        assert rows == []
        connection.commit()


def test_fetchall(connection):
    drop_and_create_table(connection)

    with connection.cursor() as cursor:
        cursor.execute("select * from tsubakuro_rust_python_test order by foo")
        assert cursor.rownumber == 0
        rows = cursor.fetchall()
        assert cursor.rownumber == 3
        assert rows == [(1, 100, "abc"), (2, 200, "def"), (3, 300, "ghi")]
        connection.commit()


def test_iteration(connection):
    drop_and_create_table(connection)
    with connection.cursor() as cursor:
        cursor.execute("select * from tsubakuro_rust_python_test order by foo")
        assert cursor.rownumber == 0

        i = 0
        for row in cursor:
            match i:
                case 0:
                    assert row == (1, 100, "abc")
                case 1:
                    assert row == (2, 200, "def")
                case 2:
                    assert row == (3, 300, "ghi")
                case _:
                    assert False, "too many rows"
            i += 1

        assert cursor.rownumber == 3
        connection.commit()


def test_closed(connection):
    with connection.cursor() as cursor:
        assert cursor.closed is False
        cursor.close()
        assert cursor.closed is True

        try:
            cursor.execute("select * from tsubakuro_rust_python_test")
        except ProgrammingError as e:
            assert str(e) == "Cursor is already closed"

        try:
            cursor.description
        except ProgrammingError as e:
            assert str(e) == "Cursor is already closed"

        try:
            cursor.fetchone()
        except ProgrammingError as e:
            assert str(e) == "Cursor is already closed"

        try:
            cursor.fetchmany()
        except ProgrammingError as e:
            assert str(e) == "Cursor is already closed"

        try:
            cursor.fetchall()
        except ProgrammingError as e:
            assert str(e) == "Cursor is already closed"

        try:
            for row in cursor:
                pass
        except ProgrammingError as e:
            assert str(e) == "Cursor is already closed"


def test_cursor_close_after_commit(connection):
    drop_and_create_table(connection)

    # tsurugi.env_logger_init("tsubakuro_rust_python=trace")

    for _ in range(100):
        try:
            with connection.cursor() as cursor:
                cursor.execute("select * from tsubakuro_rust_python_test order by foo")
                # do not fetch
                connection.commit()
        except tsurugi.error.RestrictedOperationException:
            pass
