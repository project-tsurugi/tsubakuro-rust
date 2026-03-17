import datetime
import tsurugi_dbapi as tsurugi


def drop_and_create_table(connection):
    with connection.cursor() as cursor:
        cursor.execute("drop table if exists tsubakuro_rust_python_test")
        cursor.execute(
            "create table tsubakuro_rust_python_test (pk int primary key, value time with time zone)"
        )
        connection.commit()


def test_placeholder(connection):
    drop_and_create_table(connection)

    JST = datetime.timezone(datetime.timedelta(hours=9))
    UTC = datetime.timezone(datetime.timedelta(hours=0))

    with connection.cursor() as cursor:
        parameters = [
            (1, datetime.time(0, 0, 0, tzinfo=JST)),
            (2, datetime.time(12, 34, 56, 123456, tzinfo=JST)),
            (3, None),
        ]
        cursor.executemany(
            "insert into tsubakuro_rust_python_test values (?, ?)", parameters
        )
        assert cursor.rowcount == 3

        parameters = [
            (4, datetime.time(0, 0, 0, tzinfo=UTC)),
            (5, datetime.time(12, 34, 56, 123456, tzinfo=UTC)),
        ]
        cursor.executemany(
            "insert into tsubakuro_rust_python_test values (?, ?)", parameters
        )
        assert cursor.rowcount == 2
        connection.commit()

        cursor.execute("select * from tsubakuro_rust_python_test order by pk")
        rows = cursor.fetchall()
        assert rows == [
            (1, datetime.time(15, 0, 0, tzinfo=UTC)),
            (2, datetime.time(12, 34, 56, 123456, tzinfo=JST)),
            (3, None),
            (4, datetime.time(0, 0, 0, tzinfo=UTC)),
            (5, datetime.time(12, 34, 56, 123456, tzinfo=UTC)),
        ]
        connection.commit()


def test_placeholder_wrapper(connection):
    drop_and_create_table(connection)

    JST = datetime.timezone(datetime.timedelta(hours=9))
    UTC = datetime.timezone(datetime.timedelta(hours=0))

    with connection.cursor() as cursor:
        parameters = [
            (tsurugi.type_code.Int32(0), tsurugi.type_code.OffsetTime(None)),
            (1, datetime.time(0, 0, 0, tzinfo=JST)),
            (2, datetime.time(12, 34, 56, 123456, tzinfo=JST)),
        ]
        cursor.executemany(
            "insert into tsubakuro_rust_python_test values (?, ?)", parameters
        )
        assert cursor.rowcount == 3

        parameters = [
            (3, datetime.time(0, 0, 0, tzinfo=UTC)),
            (4, datetime.time(12, 34, 56, 123456, tzinfo=UTC)),
        ]
        cursor.executemany(
            "insert into tsubakuro_rust_python_test values (?, ?)", parameters
        )
        assert cursor.rowcount == 2
        connection.commit()

        cursor.execute("select * from tsubakuro_rust_python_test order by pk")
        rows = cursor.fetchall()
        assert rows == [
            (0, None),
            (1, datetime.time(15, 0, 0, tzinfo=UTC)),
            (2, datetime.time(12, 34, 56, 123456, tzinfo=JST)),
            (3, datetime.time(0, 0, 0, tzinfo=UTC)),
            (4, datetime.time(12, 34, 56, 123456, tzinfo=UTC)),
        ]
        connection.commit()


def test_named_placeholder(connection):
    drop_and_create_table(connection)

    JST = datetime.timezone(datetime.timedelta(hours=9))
    UTC = datetime.timezone(datetime.timedelta(hours=0))

    with connection.cursor() as cursor:
        parameters = [
            {
                "pk": tsurugi.type_code.Int32(0),
                "value": tsurugi.type_code.OffsetTime(None),
            },
            {
                "pk": tsurugi.type_code.Int32(1),
                "value": tsurugi.type_code.OffsetTime(
                    datetime.time(0, 0, 0, tzinfo=JST)
                ),
            },
            {
                "pk": tsurugi.type_code.Int32(2),
                "value": tsurugi.type_code.OffsetTime(
                    datetime.time(12, 34, 56, 123456, tzinfo=JST)
                ),
            },
        ]
        cursor.executemany(
            "insert into tsubakuro_rust_python_test values (:pk, :value)", parameters
        )
        assert cursor.rowcount == 3

        parameters = [
            {"pk": 3, "value": datetime.time(0, 0, 0, tzinfo=UTC)},
            {"pk": 4, "value": datetime.time(12, 34, 56, 123456, tzinfo=UTC)},
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
            (1, datetime.time(15, 0, 0, tzinfo=UTC)),
            (2, datetime.time(12, 34, 56, 123456, tzinfo=JST)),
            (3, datetime.time(0, 0, 0, tzinfo=UTC)),
            (4, datetime.time(12, 34, 56, 123456, tzinfo=UTC)),
        ]
        connection.commit()


def test_prepare_qmark(connection):
    drop_and_create_table(connection)

    JST = datetime.timezone(datetime.timedelta(hours=9))
    UTC = datetime.timezone(datetime.timedelta(hours=0))

    with connection.cursor() as cursor:
        insert = "insert into tsubakuro_rust_python_test values (?, ?)"
        cursor.prepare(
            insert,
            (tsurugi.type_code.Int32, tsurugi.type_code.OffsetTime),
        )
        parameters = [
            (0, None),
            (1, datetime.time(0, 0, 0, tzinfo=JST)),
            (2, datetime.time(23, 59, 59, 999999, tzinfo=JST)),
        ]
        cursor.executemany(insert, parameters)
        assert cursor.rowcount == 3

        cursor.prepare(
            insert,
            (tsurugi.type_code.Int32(), tsurugi.type_code.OffsetTime()),
        )
        parameters = [
            (3, datetime.time(0, 0, 0, tzinfo=UTC)),
            (4, datetime.time(23, 59, 59, 999999, tzinfo=UTC)),
        ]
        cursor.executemany(insert, parameters)
        assert cursor.rowcount == 2
        connection.commit()

        cursor.execute("select * from tsubakuro_rust_python_test order by pk")
        rows = cursor.fetchall()
        assert rows == [
            (0, None),
            (1, datetime.time(15, 0, 0, tzinfo=UTC)),
            (2, datetime.time(23, 59, 59, 999999, tzinfo=JST)),
            (3, datetime.time(0, 0, 0, tzinfo=UTC)),
            (4, datetime.time(23, 59, 59, 999999, tzinfo=UTC)),
        ]
        connection.commit()


def test_prepare_named(connection):
    drop_and_create_table(connection)

    JST = datetime.timezone(datetime.timedelta(hours=9))
    UTC = datetime.timezone(datetime.timedelta(hours=0))

    with connection.cursor() as cursor:
        insert = "insert into tsubakuro_rust_python_test values (:pk, :value)"
        cursor.prepare(
            insert,
            {"pk": tsurugi.type_code.Int32, "value": tsurugi.type_code.OffsetTime},
        )
        parameters = [
            {"pk": 0, "value": None},
            {"pk": 1, "value": datetime.time(0, 0, 0, tzinfo=JST)},
            {"pk": 2, "value": datetime.time(23, 59, 59, 999999, tzinfo=JST)},
        ]
        cursor.executemany(insert, parameters)
        assert cursor.rowcount == 3

        cursor.prepare(
            insert,
            {"pk": tsurugi.type_code.Int32(), "value": tsurugi.type_code.OffsetTime()},
        )
        parameters = [
            {"pk": 3, "value": datetime.time(0, 0, 0, tzinfo=UTC)},
            {"pk": 4, "value": datetime.time(23, 59, 59, 999999, tzinfo=UTC)},
        ]
        cursor.executemany(insert, parameters)
        assert cursor.rowcount == 2
        connection.commit()

        cursor.execute("select * from tsubakuro_rust_python_test order by pk")
        rows = cursor.fetchall()
        assert rows == [
            (0, None),
            (1, datetime.time(15, 0, 0, tzinfo=UTC)),
            (2, datetime.time(23, 59, 59, 999999, tzinfo=JST)),
            (3, datetime.time(0, 0, 0, tzinfo=UTC)),
            (4, datetime.time(23, 59, 59, 999999, tzinfo=UTC)),
        ]
        connection.commit()
