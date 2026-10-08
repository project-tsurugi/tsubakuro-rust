import pytest
import tsurugi_dbapi as tsurugi


def test_multi_connection(endpoint):
    from concurrent.futures import ThreadPoolExecutor

    prepare(endpoint)

    with ThreadPoolExecutor(max_workers=10) as executor:
        futures = [executor.submit(execute, i, endpoint) for i in range(10)]
        for future in futures:
            future.result()


def prepare(endpoint):
    config = tsurugi.Config()
    config.application_name = "tsubakuro-rust-python-dbtest.pytest"
    config.endpoint = endpoint
    config.user = "tsurugi"
    config.password = "password"
    config.session_label = "tsubakuro-rust-python-dbteset.prepare"
    with tsurugi.connect(config) as connection, connection.cursor() as cursor:
        cursor.execute("drop table if exists tsubakuro_rust_python_test")
        cursor.execute("create table tsubakuro_rust_python_test (pk int primary key)")
        connection.commit()

        sql = "insert into tsubakuro_rust_python_test (pk) values "
        for i in range(100):
            if i > 0:
                sql += ", "
            sql += f"({i})"
        cursor.execute(sql)
        connection.commit()


def execute(number: int, endpoint):
    config = tsurugi.Config()
    config.application_name = "tsubakuro-rust-python-dbtest.pytest"
    config.endpoint = endpoint
    config.user = "tsurugi"
    config.password = "password"
    config.session_label = f"tsubakuro-rust-python-dbteset.session{number}"
    with tsurugi.connect(config) as connection, connection.cursor() as cursor:
        cursor.execute("select * from tsubakuro_rust_python_test order by pk")
        rows = cursor.fetchall()
        assert len(rows) == 100
        for i, row in enumerate(rows):
            assert row[0] == i
