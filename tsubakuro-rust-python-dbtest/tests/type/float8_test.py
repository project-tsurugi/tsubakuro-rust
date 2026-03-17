import tsurugi_dbapi as tsurugi


def drop_and_create_table(connection):
    with connection.cursor() as cursor:
        cursor.execute("drop table if exists tsubakuro_rust_python_test")
        cursor.execute(
            "create table tsubakuro_rust_python_test (pk int primary key, value double)"
        )
        connection.commit()


def test_double(connection):
    drop_and_create_table(connection)

    with connection.cursor() as cursor:
        cursor.execute(
            "insert into tsubakuro_rust_python_test values (0, null), (1, 1), (2, 0), (3, -123.5), (4, 123.5) "
        )
        connection.commit()

        metadata = connection.get_table_metadata("tsubakuro_rust_python_test")
        description = metadata.description
        assert description == (
            ("pk", "Int32", None, None, None, None, False),
            ("value", "Float64", None, None, None, None, True),
        )

        cursor.execute("select * from tsubakuro_rust_python_test order by pk")
        rows = cursor.fetchall()
        assert rows == [
            (0, None),
            (1, 1.0),
            (2, 0.0),
            (3, -123.5),
            (4, 123.5),
        ]
        assert type(rows[1][1]) is float
        description = cursor.description
        assert description == (
            ("pk", "Int32", None, None, None, None, None),
            ("value", "Float64", None, None, None, None, None),
        )
        connection.commit()


def test_placeholder(connection):
    drop_and_create_table(connection)

    with connection.cursor() as cursor:
        parameters = [
            (1, 1.0),
            (2, 0.0),
            (3, -123.5),
            (4, 123.5),
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


def test_placeholder_wrapper(connection):
    drop_and_create_table(connection)

    with connection.cursor() as cursor:
        parameters = [
            (tsurugi.type_code.Int32(0), tsurugi.type_code.Float64(None)),
            (1, 1.0),
            (2, 0.0),
            (3, -123.5),
            (4, 123.5),
        ]
        cursor.executemany(
            "insert into tsubakuro_rust_python_test values (?, ?)", parameters
        )
        assert cursor.rowcount == 5
        connection.commit()

        cursor.execute("select * from tsubakuro_rust_python_test order by pk")
        rows = cursor.fetchall()
        assert rows == [
            (0, None),
            (1, 1.0),
            (2, 0.0),
            (3, -123.5),
            (4, 123.5),
        ]
        connection.commit()


def test_named_placeholder(connection):
    drop_and_create_table(connection)

    with connection.cursor() as cursor:
        parameters = [
            {
                "pk": tsurugi.type_code.Int32(0),
                "value": tsurugi.type_code.Float64(None),
            },
            {"pk": tsurugi.type_code.Int32(1), "value": tsurugi.type_code.Float64(1.0)},
            {"pk": tsurugi.type_code.Int32(2), "value": tsurugi.type_code.Float64(0.0)},
        ]
        cursor.executemany(
            "insert into tsubakuro_rust_python_test values (:pk, :value)", parameters
        )
        assert cursor.rowcount == 3

        parameters = [
            {"pk": 3, "value": -123.5},
            {"pk": 4, "value": 123.5},
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
            (1, 1.0),
            (2, 0.0),
            (3, -123.5),
            (4, 123.5),
        ]
        connection.commit()


def test_prepare_qmark(connection):
    drop_and_create_table(connection)

    with connection.cursor() as cursor:
        insert = "insert into tsubakuro_rust_python_test values (?, ?)"
        cursor.prepare(
            insert,
            (tsurugi.type_code.Int32, tsurugi.type_code.Float64),
        )
        parameters = [
            (0, None),
            (1, 1.0),
            (2, 0.0),
        ]
        cursor.executemany(insert, parameters)
        assert cursor.rowcount == 3

        cursor.prepare(
            insert,
            (tsurugi.type_code.Int32(), tsurugi.type_code.Float64()),
        )
        parameters = [
            (3, -123.5),
            (4, 123.5),
        ]
        cursor.executemany(insert, parameters)
        assert cursor.rowcount == 2
        connection.commit()

        cursor.execute("select * from tsubakuro_rust_python_test order by pk")
        rows = cursor.fetchall()
        assert rows == [
            (0, None),
            (1, 1.0),
            (2, 0.0),
            (3, -123.5),
            (4, 123.5),
        ]


def test_prepare_named(connection):
    drop_and_create_table(connection)

    with connection.cursor() as cursor:
        insert = "insert into tsubakuro_rust_python_test values (:pk, :value)"
        cursor.prepare(
            insert,
            {"pk": tsurugi.type_code.Int32, "value": tsurugi.type_code.Float64},
        )
        parameters = [
            {"pk": 0, "value": None},
            {"pk": 1, "value": 1.0},
            {"pk": 2, "value": 0.0},
        ]
        cursor.executemany(insert, parameters)
        assert cursor.rowcount == 3

        cursor.prepare(
            insert,
            {"pk": tsurugi.type_code.Int32(), "value": tsurugi.type_code.Float64()},
        )
        parameters = [
            {"pk": 3, "value": -123.5},
            {"pk": 4, "value": 123.5},
        ]
        cursor.executemany(insert, parameters)
        assert cursor.rowcount == 2
        connection.commit()

        cursor.execute("select * from tsubakuro_rust_python_test order by pk")
        rows = cursor.fetchall()
        assert rows == [
            (0, None),
            (1, 1.0),
            (2, 0.0),
            (3, -123.5),
            (4, 123.5),
        ]


def test_numpy(connection):
    import numpy as np

    drop_and_create_table(connection)

    with connection.cursor() as cursor:
        pk_array = np.array([1, 2, 3, 4], dtype=np.int32)
        value_array = np.array([1.0, 0.0, -123.5, 123.5], dtype=np.float64)

        parameters = list(zip(pk_array, value_array))
        cursor.executemany(
            "insert into tsubakuro_rust_python_test values (?, ?)", parameters
        )
        assert cursor.rowcount == 4
        connection.commit()

        cursor.execute("select * from tsubakuro_rust_python_test order by pk")
        rows = cursor.fetchall()
        assert rows == [
            (1, 1.0),
            (2, 0.0),
            (3, -123.5),
            (4, 123.5),
        ]
        connection.commit()
