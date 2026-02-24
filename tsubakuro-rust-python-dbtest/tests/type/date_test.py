import datetime
import tsubakuro_rust_python as tsurugi


def drop_and_create_table(connection):
    with connection.cursor() as cursor:
        cursor.execute("drop table if exists tsubakuro_rust_python_test")
        cursor.execute(
            "create table tsubakuro_rust_python_test (pk int primary key, value date)"
        )
        connection.commit()


def test_date(connection):
    drop_and_create_table(connection)

    with connection.cursor() as cursor:
        cursor.execute(
            "insert into tsubakuro_rust_python_test values (0, null), (1, date'2026-01-27')"
        )
        connection.commit()

        metadata = connection.get_table_metadata("tsubakuro_rust_python_test")
        description = metadata.description
        assert description == (
            ("pk", "Int32", None, None, None, None, False),
            ("value", "Date", None, None, None, None, True),
        )

        cursor.execute("select * from tsubakuro_rust_python_test order by pk")
        rows = cursor.fetchall()
        assert rows == [
            (0, None),
            (1, datetime.date(2026, 1, 27)),
        ]
        assert type(rows[1][1]) is datetime.date
        description = cursor.description
        assert description == (
            ("pk", "Int32", None, None, None, None, None),
            ("value", "Date", None, None, None, None, None),
        )
        connection.commit()


def test_placeholder(connection):
    drop_and_create_table(connection)

    with connection.cursor() as cursor:
        parameters = [
            (1, datetime.date(2026, 1, 27)),
            (2, None),
        ]
        cursor.executemany(
            "insert into tsubakuro_rust_python_test values (?, ?)", parameters
        )
        assert cursor.rowcount == 2

        parameters = [
            (3, datetime.date(9999, 12, 31)),
        ]
        cursor.executemany(
            "insert into tsubakuro_rust_python_test values (?, ?)", parameters
        )
        assert cursor.rowcount == 1
        connection.commit()

        cursor.execute("select * from tsubakuro_rust_python_test order by pk")
        rows = cursor.fetchall()
        assert rows == [
            (1, datetime.date(2026, 1, 27)),
            (2, None),
            (3, datetime.date(9999, 12, 31)),
        ]
        connection.commit()


def test_wrapper():
    value = tsurugi.type_code.Date()
    assert value.value is None
    value = tsurugi.type_code.Date(None)
    assert value.value is None
    value = tsurugi.type_code.Date(datetime.date(2026, 1, 27))
    assert value.value == datetime.date(2026, 1, 27)

    value = tsurugi.type_code.Date.of(2026, 1, 27)
    assert value.value == datetime.date(2026, 1, 27)

    value = tsurugi.type_code.Date.raw(20480)
    assert value.value == datetime.date(2026, 1, 27)


def test_placeholder_wrapper(connection):
    drop_and_create_table(connection)

    with connection.cursor() as cursor:
        parameters = [
            (tsurugi.type_code.Int32(0), tsurugi.type_code.Date(None)),
            (1, datetime.date(2026, 1, 27)),
        ]
        cursor.executemany(
            "insert into tsubakuro_rust_python_test values (?, ?)", parameters
        )
        assert cursor.rowcount == 2

        parameters = [
            (2, tsurugi.type_code.Date(datetime.date(9999, 12, 31))),
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
            (1, datetime.date(2026, 1, 27)),
            (2, datetime.date(9999, 12, 31)),
        ]
        connection.commit()


def test_named_placeholder(connection):
    drop_and_create_table(connection)

    with connection.cursor() as cursor:
        parameters = [
            {"pk": tsurugi.type_code.Int32(0), "value": tsurugi.type_code.Date(None)},
            {
                "pk": tsurugi.type_code.Int32(1),
                "value": tsurugi.type_code.Date(datetime.date(2026, 1, 27)),
            },
        ]
        cursor.executemany(
            "insert into tsubakuro_rust_python_test values (:pk, :value)", parameters
        )
        assert cursor.rowcount == 2

        parameters = [
            {"pk": 2, "value": datetime.date(9999, 12, 31)},
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
            (1, datetime.date(2026, 1, 27)),
            (2, datetime.date(9999, 12, 31)),
        ]
        connection.commit()


def test_prepare_qmark(connection):
    drop_and_create_table(connection)

    with connection.cursor() as cursor:
        insert = "insert into tsubakuro_rust_python_test values (?, ?)"
        cursor.prepare(
            insert,
            (tsurugi.type_code.Int32, tsurugi.type_code.Date),
        )
        parameters = [
            (0, None),
            (1, datetime.date(2026, 1, 27)),
            (2, datetime.date(9999, 12, 31)),
        ]
        cursor.executemany(insert, parameters)
        assert cursor.rowcount == 3

        cursor.prepare(
            insert,
            (tsurugi.type_code.Int32(), tsurugi.type_code.Date()),
        )
        parameters = [
            (3, datetime.date(1, 1, 1)),
            (4, datetime.date(9999, 12, 31)),
        ]
        cursor.executemany(insert, parameters)
        assert cursor.rowcount == 2
        connection.commit()

        cursor.execute("select * from tsubakuro_rust_python_test order by pk")
        rows = cursor.fetchall()
        assert rows == [
            (0, None),
            (1, datetime.date(2026, 1, 27)),
            (2, datetime.date(9999, 12, 31)),
            (3, datetime.date(1, 1, 1)),
            (4, datetime.date(9999, 12, 31)),
        ]
        connection.commit()


def test_prepare_named(connection):
    drop_and_create_table(connection)

    with connection.cursor() as cursor:
        insert = "insert into tsubakuro_rust_python_test values (:pk, :value)"
        cursor.prepare(
            insert,
            {"pk": tsurugi.type_code.Int32, "value": tsurugi.type_code.Date},
        )
        parameters = [
            {"pk": 0, "value": None},
            {"pk": 1, "value": datetime.date(2026, 1, 27)},
            {"pk": 2, "value": datetime.date(9999, 12, 31)},
        ]
        cursor.executemany(insert, parameters)
        assert cursor.rowcount == 3

        cursor.prepare(
            insert,
            {"pk": tsurugi.type_code.Int32(), "value": tsurugi.type_code.Date()},
        )
        parameters = [
            {"pk": 3, "value": datetime.date(1, 1, 1)},
            {"pk": 4, "value": datetime.date(9999, 12, 31)},
        ]
        cursor.executemany(insert, parameters)
        assert cursor.rowcount == 2
        connection.commit()

        cursor.execute("select * from tsubakuro_rust_python_test order by pk")
        rows = cursor.fetchall()
        assert rows == [
            (0, None),
            (1, datetime.date(2026, 1, 27)),
            (2, datetime.date(9999, 12, 31)),
            (3, datetime.date(1, 1, 1)),
            (4, datetime.date(9999, 12, 31)),
        ]
        connection.commit()
