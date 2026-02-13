import datetime
import tsubakuro_rust_python as tsurugi


def drop_and_create_table(connection):
    with connection.cursor() as cursor:
        cursor.execute("drop table if exists tsubakuro_rust_python_test")
        cursor.execute(
            "create table tsubakuro_rust_python_test (pk int primary key, value time)"
        )
        connection.commit()


def test_time(connection):
    drop_and_create_table(connection)

    with connection.cursor() as cursor:
        cursor.execute(
            "insert into tsubakuro_rust_python_test values (0, null), (1, time'00:00:00'), (2, time'23:59:59.999999321')"
        )
        connection.commit()

        metadata = connection.get_table_metadata("tsubakuro_rust_python_test")
        description = metadata.description
        assert description == (
            ("pk", "Int32", None, None, None, None, False),
            ("value", "Time", None, None, None, None, True),
        )

        cursor.execute("select * from tsubakuro_rust_python_test order by pk")
        rows = cursor.fetchall()
        assert rows == [
            (0, None),
            (1, datetime.time(0, 0, 0)),
            (2, datetime.time(23, 59, 59, 999999)),
        ]
        assert type(rows[1][1]) is datetime.time
        description = cursor.description
        assert description == (
            ("pk", "Int32", None, None, None, None, None),
            ("value", "Time", None, None, None, None, None),
        )
        connection.commit()


def test_placeholder(connection):
    drop_and_create_table(connection)

    with connection.cursor() as cursor:
        parameters = [
            (1, datetime.time(0, 0, 0)),
            (2, datetime.time(12, 34, 56, 123456)),
            (3, None),
        ]
        cursor.executemany(
            "insert into tsubakuro_rust_python_test values (?, ?)", parameters
        )
        assert cursor.rowcount == 3

        parameters = [
            (4, datetime.time(23, 59, 59, 999999)),
        ]
        cursor.executemany(
            "insert into tsubakuro_rust_python_test values (?, ?)", parameters
        )
        assert cursor.rowcount == 1
        connection.commit()

        cursor.execute("select * from tsubakuro_rust_python_test order by pk")
        rows = cursor.fetchall()
        assert rows == [
            (1, datetime.time(0, 0, 0)),
            (2, datetime.time(12, 34, 56, 123456)),
            (3, None),
            (4, datetime.time(23, 59, 59, 999999)),
        ]
        connection.commit()


def test_wrapper():
    value = tsurugi.Time()
    assert value.value is None
    value = tsurugi.Time(None)
    assert value.value is None
    assert value.nanosecond is None

    value = tsurugi.Time(datetime.time(12, 34, 56, 123456))
    assert value.value == datetime.time(12, 34, 56, 123456)
    assert value.nanosecond == 123456000

    value = tsurugi.Time(datetime.time(12, 34, 56), 123456789)
    assert value.value == datetime.time(12, 34, 56, 123456)
    assert value.nanosecond == 123456789
    assert value.__repr__() == "Time(12:34:56.123456789)"


def test_placeholder_wrapper(connection):
    drop_and_create_table(connection)

    with connection.cursor() as cursor:
        parameters = [
            (tsurugi.Int32(0), tsurugi.Time(None)),
            (1, datetime.time(0, 0, 0)),
            (2, datetime.time(12, 34, 56, 123456)),
        ]
        cursor.executemany(
            "insert into tsubakuro_rust_python_test values (?, ?)", parameters
        )
        assert cursor.rowcount == 3

        parameters = [
            (3, datetime.time(23, 59, 59, 999999)),
        ]
        cursor.executemany(
            "insert into tsubakuro_rust_python_test values (?, ?)", parameters
        )
        assert cursor.rowcount == 1
        connection.commit()

        cursor.execute("select * from tsubakuro_rust_python_test order by pk")
        rows = cursor.fetchall()
        assert rows == [
            (0, None),
            (1, datetime.time(0, 0, 0)),
            (2, datetime.time(12, 34, 56, 123456)),
            (3, datetime.time(23, 59, 59, 999999)),
        ]
        connection.commit()


def test_named_placeholder(connection):
    drop_and_create_table(connection)

    with connection.cursor() as cursor:
        parameters = [
            {"pk": tsurugi.Int32(0), "value": tsurugi.Time(None)},
            {"pk": tsurugi.Int32(1), "value": tsurugi.Time(datetime.time(0, 0, 0))},
        ]
        cursor.executemany(
            "insert into tsubakuro_rust_python_test values (:pk, :value)", parameters
        )
        assert cursor.rowcount == 2

        parameters = [
            {"pk": 2, "value": datetime.time(23, 59, 59, 999999)},
        ]
        cursor.executemany(
            "insert into tsubakuro_rust_python_test values (:pk, :value)", parameters
        )
        assert cursor.rowcount == 1
        connection.commit()

        cursor.execute("select * from tsubakuro_rust_python_test order by pk")
        rows = cursor.fetchall()
        assert rows == [
            (0, None),
            (1, datetime.time(0, 0, 0)),
            (2, datetime.time(23, 59, 59, 999999)),
        ]
        connection.commit()


def test_prepare_qmark(connection):
    drop_and_create_table(connection)

    with connection.cursor() as cursor:
        insert = "insert into tsubakuro_rust_python_test values (?, ?)"
        cursor.prepare(
            insert,
            (tsurugi.Int32, tsurugi.Time),
        )
        parameters = [
            (0, None),
            (1, datetime.time(0, 0, 0)),
            (2, datetime.time(23, 59, 59, 999999)),
        ]
        cursor.executemany(insert, parameters)
        assert cursor.rowcount == 3

        cursor.prepare(
            insert,
            (tsurugi.Int32(), tsurugi.Time()),
        )
        parameters = [
            (3, datetime.time(0, 0, 0)),
            (4, datetime.time(23, 59, 59, 999999)),
        ]
        cursor.executemany(insert, parameters)
        assert cursor.rowcount == 2
        connection.commit()

        cursor.execute("select * from tsubakuro_rust_python_test order by pk")
        rows = cursor.fetchall()
        assert rows == [
            (0, None),
            (1, datetime.time(0, 0, 0)),
            (2, datetime.time(23, 59, 59, 999999)),
            (3, datetime.time(0, 0, 0)),
            (4, datetime.time(23, 59, 59, 999999)),
        ]
        connection.commit()


def test_prepare_named(connection):
    drop_and_create_table(connection)

    with connection.cursor() as cursor:
        insert = "insert into tsubakuro_rust_python_test values (:pk, :value)"
        cursor.prepare(
            insert,
            {"pk": tsurugi.Int32, "value": tsurugi.Time},
        )
        parameters = [
            {"pk": 0, "value": None},
            {"pk": 1, "value": datetime.time(0, 0, 0)},
            {"pk": 2, "value": datetime.time(23, 59, 59, 999999)},
        ]
        cursor.executemany(insert, parameters)
        assert cursor.rowcount == 3

        cursor.prepare(
            insert,
            {"pk": tsurugi.Int32(), "value": tsurugi.Time()},
        )
        parameters = [
            {"pk": 3, "value": datetime.time(0, 0, 0)},
            {"pk": 4, "value": datetime.time(23, 59, 59, 999999)},
        ]
        cursor.executemany(insert, parameters)
        assert cursor.rowcount == 2
        connection.commit()

        cursor.execute("select * from tsubakuro_rust_python_test order by pk")
        rows = cursor.fetchall()
        assert rows == [
            (0, None),
            (1, datetime.time(0, 0, 0)),
            (2, datetime.time(23, 59, 59, 999999)),
            (3, datetime.time(0, 0, 0)),
            (4, datetime.time(23, 59, 59, 999999)),
        ]
        connection.commit()
