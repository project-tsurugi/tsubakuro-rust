import tsubakuro_rust_python as tsurugi


def drop_and_create_table(connection):
    with connection.cursor() as cursor:
        cursor.execute("drop table if exists tsubakuro_rust_python_test")
        cursor.execute(
            "create table tsubakuro_rust_python_test (pk int primary key, value varbinary(10))"
        )
        connection.commit()


def test_varbinary(connection):
    drop_and_create_table(connection)

    with connection.cursor() as cursor:
        cursor.execute(
            "insert into tsubakuro_rust_python_test values (0, null), (1, X'123456'), (2, X'abcdef'), (3, X'')"
        )
        connection.commit()

        metadata = connection.get_table_metadata("tsubakuro_rust_python_test")
        description = metadata.description
        assert description == (
            ("pk", "Int32", None, None, None, None, False),
            ("value", "Bytes", None, 10, None, None, True),
        )

        cursor.execute("select * from tsubakuro_rust_python_test order by pk")
        rows = cursor.fetchall()
        assert rows == [
            (0, None),
            (1, b"\x12\x34\x56"),
            (2, b"\xab\xcd\xef"),
            (3, b""),
        ]
        assert type(rows[1][1]) is bytes
        description = cursor.description
        assert description == (
            ("pk", "Int32", None, None, None, None, None),
            ("value", "Bytes", None, 10, None, None, None),
        )
        connection.commit()


def test_placeholder(connection):
    drop_and_create_table(connection)

    with connection.cursor() as cursor:
        parameters = [
            (1, b"\x12\x34\x56"),
            (2, b"\xab\xcd\xef"),
            (3, b""),
            (4, None),
        ]
        cursor.executemany(
            "insert into tsubakuro_rust_python_test values (?, ?)", parameters
        )
        connection.commit()

        cursor.execute("select * from tsubakuro_rust_python_test order by pk")
        rows = cursor.fetchall()
        assert rows == [
            (1, b"\x12\x34\x56"),
            (2, b"\xab\xcd\xef"),
            (3, b""),
            (4, None),
        ]
        connection.commit()


def test_placeholder_wrapper(connection):
    drop_and_create_table(connection)

    with connection.cursor() as cursor:
        parameters = [
            (tsurugi.Int32(0), tsurugi.Bytes(None)),
            (1, b"\x12\x34\x56"),
            (2, b"\xab\xcd\xef"),
            (3, b""),
        ]
        cursor.executemany(
            "insert into tsubakuro_rust_python_test values (?, ?)", parameters
        )
        assert cursor.rowcount == 4
        connection.commit()

        cursor.execute("select * from tsubakuro_rust_python_test order by pk")
        rows = cursor.fetchall()
        assert rows == [
            (0, None),
            (1, b"\x12\x34\x56"),
            (2, b"\xab\xcd\xef"),
            (3, b""),
        ]
        connection.commit()


def test_named_placeholder(connection):
    drop_and_create_table(connection)

    with connection.cursor() as cursor:
        parameters = [
            {"pk": tsurugi.Int32(0), "value": tsurugi.Bytes(None)},
            {"pk": tsurugi.Int32(1), "value": tsurugi.Bytes(b"\x12\x34\x56")},
            {"pk": tsurugi.Int32(2), "value": tsurugi.Bytes(b"\xab\xcd\xef")},
        ]
        cursor.executemany(
            "insert into tsubakuro_rust_python_test values (:pk, :value)", parameters
        )
        assert cursor.rowcount == 3

        parameters = [
            {"pk": 3, "value": b"\x00\x01\x00"},
            {"pk": 4, "value": b""},
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
            (1, b"\x12\x34\x56"),
            (2, b"\xab\xcd\xef"),
            (3, b"\x00\x01\x00"),
            (4, b""),
        ]
        connection.commit()


def test_prepare_qmark(connection):
    drop_and_create_table(connection)

    with connection.cursor() as cursor:
        insert = "insert into tsubakuro_rust_python_test values (?, ?)"
        cursor.prepare(
            insert,
            (tsurugi.Int32, tsurugi.Bytes),
        )
        parameters = [
            (0, None),
            (1, b"\x12\x34\x56"),
        ]
        cursor.executemany(insert, parameters)
        assert cursor.rowcount == 2

        cursor.prepare(
            insert,
            (tsurugi.Int32(), tsurugi.Bytes()),
        )
        parameters = [
            (2, b"\xab\xcd\xef"),
            (3, b""),
        ]
        cursor.executemany(insert, parameters)
        assert cursor.rowcount == 2
        connection.commit()

        cursor.execute("select * from tsubakuro_rust_python_test order by pk")
        rows = cursor.fetchall()
        assert rows == [
            (0, None),
            (1, b"\x12\x34\x56"),
            (2, b"\xab\xcd\xef"),
            (3, b""),
        ]


def test_prepare_named(connection):
    drop_and_create_table(connection)

    with connection.cursor() as cursor:
        insert = "insert into tsubakuro_rust_python_test values (:pk, :value)"
        cursor.prepare(
            insert,
            {"pk": tsurugi.Int32, "value": tsurugi.Bytes},
        )
        parameters = [
            {"pk": 0, "value": None},
            {"pk": 1, "value": b"\x12\x34\x56"},
        ]
        cursor.executemany(insert, parameters)
        assert cursor.rowcount == 2

        cursor.prepare(
            insert,
            {"pk": tsurugi.Int32(), "value": tsurugi.Bytes()},
        )
        parameters = [
            {"pk": 2, "value": b"\xab\xcd\xef"},
            {"pk": 3, "value": b""},
        ]
        cursor.executemany(insert, parameters)
        assert cursor.rowcount == 2
        connection.commit()

        cursor.execute("select * from tsubakuro_rust_python_test order by pk")
        rows = cursor.fetchall()
        assert rows == [
            (0, None),
            (1, b"\x12\x34\x56"),
            (2, b"\xab\xcd\xef"),
            (3, b""),
        ]


def test_numpy(connection):
    import numpy as np

    drop_and_create_table(connection)

    with connection.cursor() as cursor:
        pk_array = np.array([1, 2, 3], dtype=np.int32)
        value_array = np.array([b"\x12\x34\x56", b"\xab\xcd\xef", b""], dtype=np.bytes_)

        parameters = list(zip(pk_array, value_array))
        cursor.executemany(
            "insert into tsubakuro_rust_python_test values (?, ?)", parameters
        )
        assert cursor.rowcount == 3
        connection.commit()

        cursor.execute("select * from tsubakuro_rust_python_test order by pk")
        rows = cursor.fetchall()
        assert rows == [
            (1, b"\x12\x34\x56"),
            (2, b"\xab\xcd\xef"),
            (3, b""),
        ]
        connection.commit()
