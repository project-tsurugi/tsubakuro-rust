import tsubakuro_rust_python as tsurugi


def drop_and_create_table(connection):
    with connection.cursor() as cursor:
        cursor.execute("drop table if exists tsubakuro_rust_python_test")
        cursor.execute(
            "create table tsubakuro_rust_python_test (pk int primary key, value bigint)"
        )
        connection.commit()


def test_bigint(connection):
    drop_and_create_table(connection)

    with connection.cursor() as cursor:
        cursor.execute(
            "insert into tsubakuro_rust_python_test values (0, null), (1, 1), (2, 0), (3, -9223372036854775808), (4, 9223372036854775807) "
        )
        connection.commit()

        metadata = connection.get_table_metadata("tsubakuro_rust_python_test")
        description = metadata.description
        assert description == (
            ("pk", "Int32", None, None, None, None, False),
            ("value", "Int64", None, None, None, None, True),
        )

        cursor.execute("select * from tsubakuro_rust_python_test order by pk")
        rows = cursor.fetchall()
        assert rows == [
            (0, None),
            (1, 1),
            (2, 0),
            (3, -9223372036854775808),
            (4, 9223372036854775807),
        ]
        assert type(rows[1][1]) is int
        description = cursor.description
        assert description == (
            ("pk", "Int32", None, None, None, None, None),
            ("value", "Int64", None, None, None, None, None),
        )
        connection.commit()


def test_placeholder(connection):
    drop_and_create_table(connection)

    with connection.cursor() as cursor:
        parameters = [
            (1, 1),
            (2, 0),
            (3, -9223372036854775808),
            (4, 9223372036854775807),
            (9, None),
        ]
        cursor.executemany(
            "insert into tsubakuro_rust_python_test values (?, ?)", parameters
        )
        connection.commit()

        cursor.execute("select * from tsubakuro_rust_python_test order by pk")
        rows = cursor.fetchall()
        assert rows == parameters
        connection.commit()


def test_wrapper():
    value = tsurugi.Int64()
    assert value.value is None
    value = tsurugi.Int64(None)
    assert value.value is None
    value = tsurugi.Int64(123)
    assert value.value == 123


def test_placeholder_wrapper(connection):
    drop_and_create_table(connection)

    with connection.cursor() as cursor:
        parameters = [
            (tsurugi.Int32(0), tsurugi.Int64(None)),
            (1, 1),
            (2, 0),
            (3, -9223372036854775808),
            (4, 9223372036854775807),
        ]
        cursor.executemany(
            "insert into tsubakuro_rust_python_test values (?, ?)", parameters
        )
        connection.commit()

        cursor.execute("select * from tsubakuro_rust_python_test order by pk")
        rows = cursor.fetchall()
        assert rows == [
            (0, None),
            (1, 1),
            (2, 0),
            (3, -9223372036854775808),
            (4, 9223372036854775807),
        ]
        connection.commit()


def test_placeholder_wrapper2(connection):
    drop_and_create_table(connection)

    with connection.cursor() as cursor:
        parameters = [
            (tsurugi.Int32(1), tsurugi.Int64(1)),
            (2, 0),
            (3, -9223372036854775808),
            (4, 9223372036854775807),
            (9, None),
        ]
        cursor.executemany(
            "insert into tsubakuro_rust_python_test values (?, ?)", parameters
        )
        connection.commit()

        cursor.execute("select * from tsubakuro_rust_python_test order by pk")
        rows = cursor.fetchall()
        assert rows == [
            (1, 1),
            (2, 0),
            (3, -9223372036854775808),
            (4, 9223372036854775807),
            (9, None),
        ]
        connection.commit()


def test_named_placeholder(connection):
    drop_and_create_table(connection)

    with connection.cursor() as cursor:
        parameters = [
            {"pk": tsurugi.Int32(0), "value": tsurugi.Int64(None)},
            {"pk": tsurugi.Int32(1), "value": tsurugi.Int64(1)},
            {"pk": tsurugi.Int32(2), "value": tsurugi.Int64(0)},
        ]
        cursor.executemany(
            "insert into tsubakuro_rust_python_test values (:pk, :value)", parameters
        )
        assert cursor.rowcount == 3

        parameters = [
            {"pk": 3, "value": -9223372036854775808},
            {"pk": 4, "value": 9223372036854775807},
        ]
        cursor.executemany(
            "insert into tsubakuro_rust_python_test values (:pk, :value)", parameters
        )
        assert cursor.rowcount == 2
        connection.commit()

        cursor.execute("select * from tsubakuro_rust_python_test order by pk")
        rows = cursor.fetchall()
        assert rows == [
            (0, None),
            (1, 1),
            (2, 0),
            (3, -9223372036854775808),
            (4, 9223372036854775807),
        ]
        connection.commit()


def test_prepare_qmark(connection):
    drop_and_create_table(connection)

    with connection.cursor() as cursor:
        insert = "insert into tsubakuro_rust_python_test values (?, ?)"
        cursor.prepare(
            insert,
            (tsurugi.Int32, tsurugi.Int64),
        )
        parameters = [
            (0, None),
            (1, 1),
            (2, 0),
        ]
        cursor.executemany(insert, parameters)
        assert cursor.rowcount == 3

        cursor.prepare(
            insert,
            (tsurugi.Int32(), tsurugi.Int64()),
        )
        parameters = [
            (3, -9223372036854775808),
            (4, 9223372036854775807),
        ]
        cursor.executemany(insert, parameters)
        assert cursor.rowcount == 2
        connection.commit()

        cursor.execute("select * from tsubakuro_rust_python_test order by pk")
        rows = cursor.fetchall()
        assert rows == [
            (0, None),
            (1, 1),
            (2, 0),
            (3, -9223372036854775808),
            (4, 9223372036854775807),
        ]
        connection.commit()


def test_prepare_named(connection):
    drop_and_create_table(connection)

    with connection.cursor() as cursor:
        insert = "insert into tsubakuro_rust_python_test values (:pk, :value)"
        cursor.prepare(
            insert,
            {"pk": tsurugi.Int32, "value": tsurugi.Int64},
        )
        parameters = [
            {"pk": 0, "value": None},
            {"pk": 1, "value": 1},
            {"pk": 2, "value": 0},
        ]
        cursor.executemany(insert, parameters)
        assert cursor.rowcount == 3

        cursor.prepare(
            insert,
            {"pk": tsurugi.Int32(), "value": tsurugi.Int64()},
        )
        parameters = [
            {"pk": 3, "value": -9223372036854775808},
            {"pk": 4, "value": 9223372036854775807},
        ]
        cursor.executemany(insert, parameters)
        assert cursor.rowcount == 2
        connection.commit()

        cursor.execute("select * from tsubakuro_rust_python_test order by pk")
        rows = cursor.fetchall()
        assert rows == [
            (0, None),
            (1, 1),
            (2, 0),
            (3, -9223372036854775808),
            (4, 9223372036854775807),
        ]
        connection.commit()


def test_numpy(connection):
    import numpy as np

    drop_and_create_table(connection)

    with connection.cursor() as cursor:
        pk_array = np.array([1, 2, 3, 4], dtype=np.int32)
        value_array = np.array(
            [1, 0, -9223372036854775808, 9223372036854775807], dtype=np.int64
        )

        parameters = list(zip(pk_array, value_array))
        cursor.executemany(
            "insert into tsubakuro_rust_python_test values (?, ?)", parameters
        )
        assert cursor.rowcount == 4
        connection.commit()

        cursor.execute("select * from tsubakuro_rust_python_test order by pk")
        rows = cursor.fetchall()
        assert rows == [
            (1, 1),
            (2, 0),
            (3, -9223372036854775808),
            (4, 9223372036854775807),
        ]
        connection.commit()
