import datetime
import tsubakuro_rust_python as tsurugi


def drop_and_create_table(connection):
    with connection.cursor() as cursor:
        cursor.execute("drop table if exists tsubakuro_rust_python_test")
        cursor.execute(
            "create table tsubakuro_rust_python_test (pk int primary key, value timestamp)"
        )
        connection.commit()


def test_timestamp(connection):
    drop_and_create_table(connection)

    with connection.cursor() as cursor:
        cursor.execute(
            "insert into tsubakuro_rust_python_test values (0, null), (1, timestamp'2026-01-27 16:24:30.123456'), (2, timestamp'2026-01-27 23:59:59.123456789')"
        )
        connection.commit()

        metadata = connection.get_table_metadata("tsubakuro_rust_python_test")
        description = metadata.description
        assert description == (
            ("pk", "Int32", None, None, None, None, False),
            ("value", "Datetime", None, None, None, None, True),
        )

        cursor.execute("select * from tsubakuro_rust_python_test order by pk")
        rows = cursor.fetchall()
        assert rows == [
            (0, None),
            (1, datetime.datetime(2026, 1, 27, 16, 24, 30, 123456)),
            (2, datetime.datetime(2026, 1, 27, 23, 59, 59, 123456)),
        ]
        assert type(rows[1][1]) is datetime.datetime
        description = cursor.description
        assert description == (
            ("pk", "Int32", None, None, None, None, None),
            ("value", "Datetime", None, None, None, None, None),
        )
        connection.commit()


def test_placeholder(connection):
    drop_and_create_table(connection)

    with connection.cursor() as cursor:
        parameters = [
            (1, datetime.datetime(2026, 1, 27, 16, 24, 30, 123456)),
            (2, datetime.datetime(2026, 1, 27, 23, 59, 59, 123456)),
            (3, None),
        ]
        cursor.executemany(
            "insert into tsubakuro_rust_python_test values (?, ?)", parameters
        )
        assert cursor.rowcount == 3

        parameters = [
            (4, datetime.datetime(9999, 12, 31, 23, 59, 59, 999999)),
        ]
        cursor.executemany(
            "insert into tsubakuro_rust_python_test values (?, ?)", parameters
        )
        assert cursor.rowcount == 1
        connection.commit()

        cursor.execute("select * from tsubakuro_rust_python_test order by pk")
        rows = cursor.fetchall()
        assert rows == [
            (1, datetime.datetime(2026, 1, 27, 16, 24, 30, 123456)),
            (2, datetime.datetime(2026, 1, 27, 23, 59, 59, 123456)),
            (3, None),
            (4, datetime.datetime(9999, 12, 31, 23, 59, 59, 999999)),
        ]
        connection.commit()


def test_wrapper():
    value = tsurugi.Datetime()
    assert value.value is None
    value = tsurugi.Datetime(None)
    assert value.value is None
    assert value.nanosecond is None

    value = tsurugi.Datetime(datetime.datetime(2026, 1, 27, 16, 24, 30, 123456))
    assert value.value == datetime.datetime(2026, 1, 27, 16, 24, 30, 123456)
    assert value.nanosecond == 123456000
    value = tsurugi.Datetime(datetime.datetime(2026, 1, 27, 16, 24, 30), 123456789)
    assert value.value == datetime.datetime(2026, 1, 27, 16, 24, 30, 123456)
    assert value.nanosecond == 123456789
    assert value.__repr__() == "Datetime(2026-01-27 16:24:30.123456789)"

    value = tsurugi.Datetime.of(2026, 1, 27, 16, 24, 30, 123456789)
    assert value.value == datetime.datetime(2026, 1, 27, 16, 24, 30, 123456)
    assert value.nanosecond == 123456789
    value = tsurugi.Datetime.of(2026, 1, 27)
    assert value.value == datetime.datetime(2026, 1, 27, 0, 0, 0, 0)
    assert value.nanosecond == 0


def test_placeholder_wrapper(connection):
    drop_and_create_table(connection)

    with connection.cursor() as cursor:
        parameters = [
            (tsurugi.Int32(0), tsurugi.Datetime(None)),
            (1, datetime.datetime(2026, 1, 27, 16, 24, 30, 123456)),
            (2, datetime.datetime(2026, 1, 27, 23, 59, 59, 123456)),
        ]
        cursor.executemany(
            "insert into tsubakuro_rust_python_test values (?, ?)", parameters
        )
        assert cursor.rowcount == 3

        parameters = [
            (3, datetime.datetime(9999, 12, 31, 23, 59, 59, 999999)),
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
            (1, datetime.datetime(2026, 1, 27, 16, 24, 30, 123456)),
            (2, datetime.datetime(2026, 1, 27, 23, 59, 59, 123456)),
            (3, datetime.datetime(9999, 12, 31, 23, 59, 59, 999999)),
        ]
        connection.commit()


def test_named_placeholder(connection):
    drop_and_create_table(connection)

    with connection.cursor() as cursor:
        parameters = [
            {"pk": tsurugi.Int32(0), "value": tsurugi.Datetime(None)},
            {
                "pk": tsurugi.Int32(1),
                "value": tsurugi.Datetime(
                    datetime.datetime(2026, 1, 27, 16, 24, 30, 123456)
                ),
            },
        ]
        cursor.executemany(
            "insert into tsubakuro_rust_python_test values (:pk, :value)", parameters
        )
        assert cursor.rowcount == 2

        parameters = [
            {"pk": 2, "value": datetime.datetime(2026, 1, 27, 23, 59, 59, 999999)},
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
            (1, datetime.datetime(2026, 1, 27, 16, 24, 30, 123456)),
            (2, datetime.datetime(2026, 1, 27, 23, 59, 59, 999999)),
        ]
        connection.commit()


def test_prepare_qmark(connection):
    drop_and_create_table(connection)

    with connection.cursor() as cursor:
        insert = "insert into tsubakuro_rust_python_test values (?, ?)"
        cursor.prepare(
            insert,
            (tsurugi.Int32, tsurugi.Datetime),
        )
        parameters = [
            (0, None),
            (1, datetime.datetime(2026, 1, 27, 16, 24, 30, 123456)),
            (2, datetime.datetime(2026, 1, 27, 23, 59, 59, 999999)),
        ]
        cursor.executemany(insert, parameters)
        assert cursor.rowcount == 3

        cursor.prepare(
            insert,
            (tsurugi.Int32(), tsurugi.Datetime()),
        )
        parameters = [
            (3, datetime.datetime(2026, 1, 27, 16, 24, 30, 123456)),
            (4, datetime.datetime(2026, 1, 27, 23, 59, 59, 999999)),
        ]
        cursor.executemany(insert, parameters)
        assert cursor.rowcount == 2
        connection.commit()

        cursor.execute("select * from tsubakuro_rust_python_test order by pk")
        rows = cursor.fetchall()
        assert rows == [
            (0, None),
            (1, datetime.datetime(2026, 1, 27, 16, 24, 30, 123456)),
            (2, datetime.datetime(2026, 1, 27, 23, 59, 59, 999999)),
            (3, datetime.datetime(2026, 1, 27, 16, 24, 30, 123456)),
            (4, datetime.datetime(2026, 1, 27, 23, 59, 59, 999999)),
        ]
        connection.commit()


def test_prepare_named(connection):
    drop_and_create_table(connection)

    with connection.cursor() as cursor:
        insert = "insert into tsubakuro_rust_python_test values (:pk, :value)"
        cursor.prepare(
            insert,
            {"pk": tsurugi.Int32, "value": tsurugi.Datetime},
        )
        parameters = [
            {"pk": 0, "value": None},
            {"pk": 1, "value": datetime.datetime(2026, 1, 27, 16, 24, 30, 123456)},
            {"pk": 2, "value": datetime.datetime(2026, 1, 27, 23, 59, 59, 999999)},
        ]
        cursor.executemany(insert, parameters)
        assert cursor.rowcount == 3

        cursor.prepare(
            insert,
            {"pk": tsurugi.Int32(), "value": tsurugi.Datetime()},
        )
        parameters = [
            {"pk": 3, "value": datetime.datetime(2026, 1, 27, 16, 24, 30, 123456)},
            {"pk": 4, "value": datetime.datetime(2026, 1, 27, 23, 59, 59, 999999)},
        ]
        cursor.executemany(insert, parameters)
        assert cursor.rowcount == 2
        connection.commit()

        cursor.execute("select * from tsubakuro_rust_python_test order by pk")
        rows = cursor.fetchall()
        assert rows == [
            (0, None),
            (1, datetime.datetime(2026, 1, 27, 16, 24, 30, 123456)),
            (2, datetime.datetime(2026, 1, 27, 23, 59, 59, 999999)),
            (3, datetime.datetime(2026, 1, 27, 16, 24, 30, 123456)),
            (4, datetime.datetime(2026, 1, 27, 23, 59, 59, 999999)),
        ]
        connection.commit()


def test_numpy(connection):
    import numpy as np

    drop_and_create_table(connection)

    with connection.cursor() as cursor:
        pk_array = np.array([1, 2, 3], dtype=np.int32)
        value_array = np.array(
            [
                np.datetime64("2026-01-27T16:24:30.123456"),
                np.datetime64("2026-01-27T23:59:59.999999"),
                np.datetime64("1970-01-01T00:00:00.123456789"),
            ],
            dtype=np.datetime64,
        )

        parameters = list(zip(pk_array, value_array))
        cursor.executemany(
            "insert into tsubakuro_rust_python_test values (?, ?)", parameters
        )
        assert cursor.rowcount == 3
        connection.commit()

        cursor.execute("select * from tsubakuro_rust_python_test order by pk")
        rows = cursor.fetchall()
        assert rows == [
            (1, datetime.datetime(2026, 1, 27, 16, 24, 30, 123456)),
            (2, datetime.datetime(2026, 1, 27, 23, 59, 59, 999999)),
            (3, datetime.datetime(1970, 1, 1, 0, 0, 0, 123456)),
        ]
        connection.commit()
