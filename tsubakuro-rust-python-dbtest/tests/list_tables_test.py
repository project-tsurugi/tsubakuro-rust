from tsubakuro_rust_python import ProgrammingError


def test_list_tables(connection):
    with connection.cursor() as cursor:
        cursor.execute("drop table if exists tsubakuro_rust_python_test")
        connection.commit()

        table_names = connection.list_tables()
        assert "tsubakuro_rust_python_test" not in table_names

        cursor.execute("create table tsubakuro_rust_python_test (pk int primary key)")
        connection.commit()

        table_names = connection.list_tables()
        assert "tsubakuro_rust_python_test" in table_names


def test_closed(connection):
    connection.close()
    try:
        connection.list_tables()
    except ProgrammingError as e:
        assert str(e) == "Connection is already closed"
