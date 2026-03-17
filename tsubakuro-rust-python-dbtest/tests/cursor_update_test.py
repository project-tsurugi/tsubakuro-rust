from tsurugi_dbapi import ProgrammingError
from tsurugi_dbapi.type_code import Int32, Int64, Str, Decimal


def test_insert(connection):
    with connection.cursor() as cursor:
        cursor.execute("drop table if exists tsubakuro_rust_python_test")
        cursor.execute("create table tsubakuro_rust_python_test (pk int primary key)")
        connection.commit()

        cursor.execute("insert into tsubakuro_rust_python_test values (1), (2), (9)")
        assert cursor.rowcount == 3
        assert cursor.rownumber is None
        connection.commit()

        cursor.execute("select * from tsubakuro_rust_python_test order by pk")
        rows = cursor.fetchall()
        assert rows == [(1,), (2,), (9,)]
        connection.commit()


def test_execute_parameters(connection):
    with connection.cursor() as cursor:
        cursor.execute("drop table if exists tsubakuro_rust_python_test")
        cursor.execute(
            "create table tsubakuro_rust_python_test ("
            " foo int primary key,"
            " bar bigint,"
            " zzz varchar(10)"
            ")"
        )
        connection.commit()

        insert = "insert into tsubakuro_rust_python_test values (?, ?, ?)"
        cursor.execute(insert, (Int32(1), 100, "abc"))
        cursor.execute(insert, (Int32(2), 200, "def"))
        cursor.execute(insert, (Int32(9), 900, "xyz"))
        connection.commit()

        cursor.execute("select * from tsubakuro_rust_python_test order by foo")
        rows = cursor.fetchall()
        assert rows == [(1, 100, "abc"), (2, 200, "def"), (9, 900, "xyz")]
        connection.commit()


def test_executemany(connection):
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

        cursor.executemany(
            "insert into tsubakuro_rust_python_test values (?, ?, ?)",
            [(Int32(1), 100, "abc")],
        )
        assert cursor.rowcount == 1
        cursor.executemany(
            "insert into tsubakuro_rust_python_test values (?, ?, ?)",
            [(Int32(2), 200, "def"), (Int32(9), 900, "xyz")],
        )
        assert cursor.rowcount == 2
        connection.commit()

        cursor.execute("select * from tsubakuro_rust_python_test order by foo")
        rows = cursor.fetchall()
        assert rows == [(1, 100, "abc"), (2, 200, "def"), (9, 900, "xyz")]
        connection.commit()

        # wrapper test
        cursor.execute("delete from tsubakuro_rust_python_test")
        connection.commit()

        cursor.executemany(
            "insert into tsubakuro_rust_python_test values (?, ?, ?)",
            [
                (Int32(1), Int64(100), Str("abc")),
                (Int32(2), Int64(200), Str("def")),
                (Int32(9), Int64(900), Str("xyz")),
            ],
        )
        assert cursor.rowcount == 3
        connection.commit()

        cursor.execute("select * from tsubakuro_rust_python_test order by foo")
        rows = cursor.fetchall()
        assert rows == [(1, 100, "abc"), (2, 200, "def"), (9, 900, "xyz")]
        connection.commit()

        # null test
        cursor.execute("delete from tsubakuro_rust_python_test")
        connection.commit()

        cursor.executemany(
            "insert into tsubakuro_rust_python_test values (?, ?, ?)",
            [
                (Int32(1), Int64(None), Str(None)),
                (2, 200, "def"),
            ],
        )
        assert cursor.rowcount == 2
        connection.commit()

        cursor.execute("select * from tsubakuro_rust_python_test order by foo")
        rows = cursor.fetchall()
        assert rows == [(1, None, None), (2, 200, "def")]
        connection.commit()


def test_executemany_async(connection):
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

        assert cursor.executemany_async is True
        cursor.executemany_async = False

        cursor.executemany(
            "insert into tsubakuro_rust_python_test values (?, ?, ?)",
            [(Int32(1), 100, "abc")],
        )
        assert cursor.rowcount == 1
        cursor.executemany(
            "insert into tsubakuro_rust_python_test values (?, ?, ?)",
            [(Int32(2), 200, "def"), (Int32(9), 900, "xyz")],
        )
        assert cursor.rowcount == 2
        connection.commit()

        cursor.execute("select * from tsubakuro_rust_python_test order by foo")
        rows = cursor.fetchall()
        assert rows == [(1, 100, "abc"), (2, 200, "def"), (9, 900, "xyz")]
        connection.commit()


def test_prepare(connection):
    with connection.cursor() as cursor:
        cursor.execute("drop table if exists tsubakuro_rust_python_test")
        cursor.execute(
            "create table tsubakuro_rust_python_test ("
            " foo int primary key,"
            " bar decimal(5,1),"
            " zzz varchar(10)"
            ")"
        )
        connection.commit()

        insert = "insert into tsubakuro_rust_python_test values (?, ?, ?)"
        cursor.prepare(insert, (Int32, Decimal, Str))
        cursor.execute(insert, (1, 100, "abc"))
        cursor.executemany(insert, [(2, 200, "def"), (9, 900, "xyz")])
        connection.commit()

        cursor.execute("select * from tsubakuro_rust_python_test order by foo")
        rows = cursor.fetchall()
        assert rows == [(1, 100, "abc"), (2, 200, "def"), (9, 900, "xyz")]

        select = "select * from tsubakuro_rust_python_test where bar = ?"
        cursor.prepare(select, (Decimal,))
        cursor.execute(select, (200,))
        row = cursor.fetchone()
        assert row == (2, 200, "def")

        connection.commit()


def test_closed(connection):
    with connection.cursor() as cursor:
        assert cursor.closed is False
        cursor.close()
        assert cursor.closed is True

        try:
            cursor.execute("insert into tsubakuro_rust_python_test values (1)")
        except ProgrammingError as e:
            assert str(e) == "Cursor is already closed"

        try:
            cursor.executemany(
                "insert into tsubakuro_rust_python_test values (?)", [(1,)]
            )
        except ProgrammingError as e:
            assert str(e) == "Cursor is already closed"
