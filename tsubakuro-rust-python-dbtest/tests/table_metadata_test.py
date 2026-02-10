import tsubakuro_rust_python as tsurugi
from tsubakuro_rust_python import ProgrammingError


def test_table_metadata(connection):
    with connection.cursor() as cursor:
        cursor.execute("drop table if exists tsubakuro_rust_python_test")
        connection.commit()

        try:
            metadata = connection.get_table_metadata("tsubakuro_rust_python_test")
            assert False, "Expected TargetNotFoundException"
        except tsurugi.TargetNotFoundException:
            pass

        metadata = connection.find_table_metadata("tsubakuro_rust_python_test")
        assert metadata is None

        cursor.execute(
            "/** test table */"
            "create table tsubakuro_rust_python_test ("
            "/** pk */"
            " foo int primary key,"
            "/** long value */"
            " bar bigint,"
            "/** string value */"
            " zzz varchar(10)"
            ")"
        )
        connection.commit()

        metadata = connection.get_table_metadata("tsubakuro_rust_python_test")
        assert metadata.table_name == "tsubakuro_rust_python_test"
        assert metadata.table_description == "test table"
        assert metadata.primary_keys == ["foo"]

        columns = metadata.columns
        assert len(columns) == 3
        c = columns[0]
        assert c.name == "foo"
        assert c.description == "pk"
        assert c.type_code == "Int32"
        assert c.sql_type == "INT"
        c = columns[1]
        assert c.name == "bar"
        assert c.description == "long value"
        assert c.type_code == "Int64"
        assert c.sql_type == "BIGINT"
        c = columns[2]
        assert c.name == "zzz"
        assert c.description == "string value"
        assert c.type_code == "Str"
        assert c.sql_type == "VARCHAR(10)"

        desc = metadata.description
        assert desc == (
            ("foo", "Int32", None, None, None, None, False),
            ("bar", "Int64", None, None, None, None, True),
            ("zzz", "Str", None, 10, None, None, True),
        )

        metadata = connection.find_table_metadata("tsubakuro_rust_python_test")
        assert metadata.table_name == "tsubakuro_rust_python_test"
        desc = metadata.description
        assert desc == (
            ("foo", "Int32", None, None, None, None, False),
            ("bar", "Int64", None, None, None, None, True),
            ("zzz", "Str", None, 10, None, None, True),
        )


def test_closed(connection):
    connection.close()
    try:
        connection.get_table_metadata("tsubakuro_rust_python_test")
    except ProgrammingError as e:
        assert str(e) == "Connection is already closed"

    try:
        connection.find_table_metadata("tsubakuro_rust_python_test")
    except ProgrammingError as e:
        assert str(e) == "Connection is already closed"
