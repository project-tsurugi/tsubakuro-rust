import datetime
import tsubakuro_rust_python as tsurugi


def drop_and_create_table(connection):
    with connection.cursor() as cursor:
        cursor.execute("drop table if exists tsubakuro_rust_python_test")
        cursor.execute(
            "create table tsubakuro_rust_python_test (pk int primary key, value timestamp with time zone)"
        )
        connection.commit()


def test_timestamp_tz(connection):
    drop_and_create_table(connection)

    with connection.cursor() as cursor:
        cursor.execute(
            "insert into tsubakuro_rust_python_test values (0, null), (1, timestamp with time zone'2026-01-27 16:24:30.123456+09:00'), (2, timestamp with time zone'2026-01-27 23:59:59.123456789+09:00'), (3, timestamp with time zone'2026-01-27 16:24:30.123456+00:00')"
        )
        connection.commit()

        metadata = connection.get_table_metadata("tsubakuro_rust_python_test")
        description = metadata.description
        assert description == (
            ("pk", "Int32", None, None, None, None, False),
            ("value", "OffsetDatetime", None, None, None, None, True),
        )

        cursor.execute("select * from tsubakuro_rust_python_test order by pk")
        rows = cursor.fetchall()
        JST = datetime.timezone(datetime.timedelta(hours=9))
        UTC = datetime.timezone(datetime.timedelta(hours=0))
        assert rows == [
            (0, None),
            (1, datetime.datetime(2026, 1, 27, 16, 24, 30, 123456, tzinfo=JST)),
            (2, datetime.datetime(2026, 1, 27, 23, 59, 59, 123456, tzinfo=JST)),
            (3, datetime.datetime(2026, 1, 27, 16, 24, 30, 123456, tzinfo=UTC)),
        ]
        assert type(rows[1][1]) is datetime.datetime
        description = cursor.description
        assert description == (
            ("pk", "Int32", None, None, None, None, None),
            ("value", "OffsetDatetime", None, None, None, None, None),
        )
        connection.commit()


def test_placeholder(connection):
    drop_and_create_table(connection)

    JST = datetime.timezone(datetime.timedelta(hours=9))
    UTC = datetime.timezone(datetime.timedelta(hours=0))

    with connection.cursor() as cursor:
        parameters = [
            (1, datetime.datetime(2026, 1, 27, 16, 24, 30, 123456, tzinfo=JST)),
            (2, datetime.datetime(2026, 1, 27, 23, 59, 59, 123456, tzinfo=JST)),
            (3, None),
        ]
        cursor.executemany(
            "insert into tsubakuro_rust_python_test values (?, ?)", parameters
        )
        assert cursor.rowcount == 3

        parameters = [
            (4, datetime.datetime(2026, 1, 27, 16, 24, 30, 123456, tzinfo=UTC)),
            (5, datetime.datetime(2026, 1, 27, 23, 59, 59, 123456, tzinfo=UTC)),
        ]
        cursor.executemany(
            "insert into tsubakuro_rust_python_test values (?, ?)", parameters
        )
        assert cursor.rowcount == 2
        connection.commit()

        cursor.execute("select * from tsubakuro_rust_python_test order by pk")
        rows = cursor.fetchall()
        assert rows == [
            (1, datetime.datetime(2026, 1, 27, 16, 24, 30, 123456, tzinfo=JST)),
            (2, datetime.datetime(2026, 1, 27, 23, 59, 59, 123456, tzinfo=JST)),
            (3, None),
            (4, datetime.datetime(2026, 1, 27, 16, 24, 30, 123456, tzinfo=UTC)),
            (5, datetime.datetime(2026, 1, 27, 23, 59, 59, 123456, tzinfo=UTC)),
        ]
        connection.commit()


def test_placeholder_wrapper(connection):
    drop_and_create_table(connection)

    JST = datetime.timezone(datetime.timedelta(hours=9))
    UTC = datetime.timezone(datetime.timedelta(hours=0))

    with connection.cursor() as cursor:
        parameters = [
            (tsurugi.type_code.Int32(0), tsurugi.type_code.OffsetDatetime(None)),
            (1, datetime.datetime(2026, 1, 27, 16, 24, 30, 123456, tzinfo=JST)),
            (2, datetime.datetime(2026, 1, 27, 23, 59, 59, 123456, tzinfo=JST)),
        ]
        cursor.executemany(
            "insert into tsubakuro_rust_python_test values (?, ?)", parameters
        )
        assert cursor.rowcount == 3

        parameters = [
            (3, datetime.datetime(2026, 1, 27, 16, 24, 30, 123456, tzinfo=UTC)),
            (4, datetime.datetime(2026, 1, 27, 23, 59, 59, 123456, tzinfo=UTC)),
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
            (1, datetime.datetime(2026, 1, 27, 16, 24, 30, 123456, tzinfo=JST)),
            (2, datetime.datetime(2026, 1, 27, 23, 59, 59, 123456, tzinfo=JST)),
            (3, datetime.datetime(2026, 1, 27, 16, 24, 30, 123456, tzinfo=UTC)),
            (4, datetime.datetime(2026, 1, 27, 23, 59, 59, 123456, tzinfo=UTC)),
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
                "value": tsurugi.type_code.OffsetDatetime(None),
            },
            {
                "pk": tsurugi.type_code.Int32(1),
                "value": tsurugi.type_code.OffsetDatetime(
                    datetime.datetime(2026, 1, 27, 16, 24, 30, 123456, tzinfo=JST)
                ),
            },
            {
                "pk": tsurugi.type_code.Int32(2),
                "value": tsurugi.type_code.OffsetDatetime(
                    datetime.datetime(2026, 1, 27, 23, 59, 59, 123456, tzinfo=JST)
                ),
            },
        ]
        cursor.executemany(
            "insert into tsubakuro_rust_python_test values (:pk, :value)", parameters
        )
        assert cursor.rowcount == 3

        parameters = [
            {
                "pk": 3,
                "value": datetime.datetime(2026, 1, 27, 16, 24, 30, 123456, tzinfo=UTC),
            },
            {
                "pk": 4,
                "value": datetime.datetime(2026, 1, 27, 23, 59, 59, 123456, tzinfo=UTC),
            },
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
            (1, datetime.datetime(2026, 1, 27, 16, 24, 30, 123456, tzinfo=JST)),
            (2, datetime.datetime(2026, 1, 27, 23, 59, 59, 123456, tzinfo=JST)),
            (3, datetime.datetime(2026, 1, 27, 16, 24, 30, 123456, tzinfo=UTC)),
            (4, datetime.datetime(2026, 1, 27, 23, 59, 59, 123456, tzinfo=UTC)),
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
            (tsurugi.type_code.Int32, tsurugi.type_code.OffsetDatetime),
        )
        parameters = [
            (0, None),
            (1, datetime.datetime(2026, 1, 27, 16, 24, 30, 123456, tzinfo=JST)),
            (2, datetime.datetime(2026, 1, 27, 23, 59, 59, 123456, tzinfo=JST)),
        ]
        cursor.executemany(insert, parameters)
        assert cursor.rowcount == 3

        cursor.prepare(
            insert,
            (tsurugi.type_code.Int32(), tsurugi.type_code.OffsetDatetime()),
        )
        parameters = [
            (3, datetime.datetime(2026, 1, 27, 16, 24, 30, 123456, tzinfo=UTC)),
            (4, datetime.datetime(2026, 1, 27, 23, 59, 59, 123456, tzinfo=UTC)),
        ]
        cursor.executemany(insert, parameters)
        assert cursor.rowcount == 2
        connection.commit()

        cursor.execute("select * from tsubakuro_rust_python_test order by pk")
        rows = cursor.fetchall()
        assert rows == [
            (0, None),
            (1, datetime.datetime(2026, 1, 27, 16, 24, 30, 123456, tzinfo=JST)),
            (2, datetime.datetime(2026, 1, 27, 23, 59, 59, 123456, tzinfo=JST)),
            (3, datetime.datetime(2026, 1, 27, 16, 24, 30, 123456, tzinfo=UTC)),
            (4, datetime.datetime(2026, 1, 27, 23, 59, 59, 123456, tzinfo=UTC)),
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
            {"pk": tsurugi.type_code.Int32, "value": tsurugi.type_code.OffsetDatetime},
        )
        parameters = [
            {"pk": 0, "value": None},
            {
                "pk": 1,
                "value": datetime.datetime(2026, 1, 27, 16, 24, 30, 123456, tzinfo=JST),
            },
            {
                "pk": 2,
                "value": datetime.datetime(2026, 1, 27, 23, 59, 59, 123456, tzinfo=JST),
            },
        ]
        cursor.executemany(insert, parameters)
        assert cursor.rowcount == 3

        cursor.prepare(
            insert,
            {
                "pk": tsurugi.type_code.Int32(),
                "value": tsurugi.type_code.OffsetDatetime(),
            },
        )
        parameters = [
            {
                "pk": 3,
                "value": datetime.datetime(2026, 1, 27, 16, 24, 30, 123456, tzinfo=UTC),
            },
            {
                "pk": 4,
                "value": datetime.datetime(2026, 1, 27, 23, 59, 59, 123456, tzinfo=UTC),
            },
        ]
        cursor.executemany(insert, parameters)
        assert cursor.rowcount == 2
        connection.commit()

        cursor.execute("select * from tsubakuro_rust_python_test order by pk")
        rows = cursor.fetchall()
        assert rows == [
            (0, None),
            (1, datetime.datetime(2026, 1, 27, 16, 24, 30, 123456, tzinfo=JST)),
            (2, datetime.datetime(2026, 1, 27, 23, 59, 59, 123456, tzinfo=JST)),
            (3, datetime.datetime(2026, 1, 27, 16, 24, 30, 123456, tzinfo=UTC)),
            (4, datetime.datetime(2026, 1, 27, 23, 59, 59, 123456, tzinfo=UTC)),
        ]
        connection.commit()
