import tsubakuro_rust_python as tsurugi


def test_bool(connection):
    with connection.cursor() as cursor:
        cursor.execute("drop table if exists tsubakuro_rust_python_test")
        cursor.execute(
            "create table tsubakuro_rust_python_test (pk int primary key, value int)"
        )
        connection.commit()

        cursor.execute(
            "insert into tsubakuro_rust_python_test values (0, null), (1, 1), (2, 0)"
        )
        connection.commit()

        metadata = connection.get_table_metadata("tsubakuro_rust_python_test")
        description = metadata.description
        assert description == (
            ("pk", "Int32", None, None, None, None, False),
            ("value", "Int32", None, None, None, None, True),  # TODO: Bool
        )

        cursor.execute(
            "select pk, value <> 0 value from tsubakuro_rust_python_test order by pk"
        )
        rows = cursor.fetchall()
        assert rows == [
            (0, None),
            (1, True),
            (2, False),
        ]
        assert type(rows[1][1]) is bool
        description = cursor.description
        assert description == (
            ("pk", "Int32", None, None, None, None, None),
            ("value", "Bool", None, None, None, None, None),
        )
        connection.commit()


def test_wrapper():
    value = tsurugi.Bool()
    assert value.value is None
    value = tsurugi.Bool(None)
    assert value.value is None
    value = tsurugi.Bool(True)
    assert value.value is True
    value = tsurugi.Bool(False)
    assert value.value is False
