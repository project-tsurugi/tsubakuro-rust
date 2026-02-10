import tsubakuro_rust_python as tsurugi
from tsubakuro_rust_python import TransactionOption, TransactionType


def test_default():
    option = TransactionOption()
    assert option.transaction_type == TransactionType.OCC
    assert option.label is None
    assert option.include_ddl is False
    assert option.write_preserve is None
    assert option.inclusive_read_area is None
    assert option.exclusive_read_area is None
    assert option.scan_parallel is None
    assert option.begin_timeout is None

    option.label = "my_transaction"
    assert option.label == "my_transaction"


def test_config_occ(endpoint):
    config = tsurugi.Config(endpoint=endpoint, user="tsurugi", password="password")
    config.transaction_option = TransactionOption(TransactionType.OCC)
    with tsurugi.connect(config) as connection:
        with connection.cursor() as cursor:
            cursor.execute("drop table if exists tsubakuro_rust_python_test")
            cursor.execute(
                "create table tsubakuro_rust_python_test (pk int primary key)"
            )
            connection.commit()

            cursor.execute(
                "insert into tsubakuro_rust_python_test values (1), (2), (9)"
            )
            assert cursor.rowcount == 3
            connection.commit()

            cursor.execute("select * from tsubakuro_rust_python_test order by pk")
            rows = cursor.fetchall()
            assert rows == [(1,), (2,), (9,)]
            connection.commit()


def test_config_ltx(endpoint):
    config = tsurugi.Config(endpoint=endpoint, user="tsurugi", password="password")
    tx_option = TransactionOption(TransactionType.LTX)
    tx_option.include_ddl = True
    tx_option.begin_timeout = 60
    config.transaction_option = tx_option
    with tsurugi.connect(config) as connection:
        with connection.cursor() as cursor:
            cursor.execute("drop table if exists tsubakuro_rust_python_test")
            cursor.execute(
                "create table tsubakuro_rust_python_test (pk int primary key)"
            )
            connection.commit()

    tx_option = TransactionOption(TransactionType.LTX)
    tx_option.write_preserve = ["tsubakuro_rust_python_test"]
    config.transaction_option = tx_option
    with tsurugi.connect(config) as connection:
        with connection.cursor() as cursor:
            cursor.execute(
                "insert into tsubakuro_rust_python_test values (1), (2), (9)"
            )
            assert cursor.rowcount == 3
            connection.commit()

    tx_option = TransactionOption(TransactionType.LTX)
    tx_option.inclusive_read_area = ["tsubakuro_rust_python_test"]
    config.transaction_option = tx_option
    with tsurugi.connect(config) as connection:
        with connection.cursor() as cursor:
            cursor.execute("select * from tsubakuro_rust_python_test order by pk")
            rows = cursor.fetchall()
            assert rows == [(1,), (2,), (9,)]
            connection.commit()


def test_config_rtx(endpoint):
    config = tsurugi.Config(endpoint=endpoint, user="tsurugi", password="password")
    with tsurugi.connect(config) as connection:
        with connection.cursor() as cursor:
            cursor.execute("drop table if exists tsubakuro_rust_python_test")
            cursor.execute(
                "create table tsubakuro_rust_python_test (pk int primary key)"
            )
            connection.commit()

            cursor.execute(
                "insert into tsubakuro_rust_python_test values (1), (2), (9)"
            )
            assert cursor.rowcount == 3
            connection.commit()

    config = tsurugi.Config(endpoint=endpoint, user="tsurugi", password="password")
    tx_option = TransactionOption(TransactionType.RTX)
    tx_option.scan_parallel = 2
    config.transaction_option = tx_option
    with tsurugi.connect(config) as connection:
        with connection.cursor() as cursor:
            cursor.execute("select * from tsubakuro_rust_python_test order by pk")
            rows = cursor.fetchall()
            assert rows == [(1,), (2,), (9,)]
            connection.commit()


def test_connection_occ(connection):
    connection.transaction_option = TransactionOption(TransactionType.OCC)
    with connection.cursor() as cursor:
        cursor.execute("drop table if exists tsubakuro_rust_python_test")
        cursor.execute("create table tsubakuro_rust_python_test (pk int primary key)")
        connection.commit()

        cursor.execute("insert into tsubakuro_rust_python_test values (1), (2), (9)")
        assert cursor.rowcount == 3
        connection.commit()

        cursor.execute("select * from tsubakuro_rust_python_test order by pk")
        rows = cursor.fetchall()
        assert rows == [(1,), (2,), (9,)]
        connection.commit()


def test_connection_ltx(connection):
    with connection.cursor() as cursor:
        tx_option = TransactionOption(TransactionType.LTX)
        tx_option.include_ddl = True
        connection.transaction_option = tx_option
        cursor.execute("drop table if exists tsubakuro_rust_python_test")
        cursor.execute("create table tsubakuro_rust_python_test (pk int primary key)")
        connection.commit()

        tx_option = TransactionOption(TransactionType.LTX)
        tx_option.write_preserve = ["tsubakuro_rust_python_test"]
        connection.transaction_option = tx_option
        cursor.execute("insert into tsubakuro_rust_python_test values (1), (2), (9)")
        assert cursor.rowcount == 3
        connection.commit()

        tx_option = TransactionOption(TransactionType.LTX)
        tx_option.inclusive_read_area = ["tsubakuro_rust_python_test"]
        connection.transaction_option = tx_option
        cursor.execute("select * from tsubakuro_rust_python_test order by pk")
        rows = cursor.fetchall()
        assert rows == [(1,), (2,), (9,)]
        connection.commit()


def test_connection_rtx(connection):
    with connection.cursor() as cursor:
        cursor.execute("drop table if exists tsubakuro_rust_python_test")
        cursor.execute("create table tsubakuro_rust_python_test (pk int primary key)")
        connection.commit()

        cursor.execute("insert into tsubakuro_rust_python_test values (1), (2), (9)")
        assert cursor.rowcount == 3
        connection.commit()

        tx_option = TransactionOption(TransactionType.RTX)
        tx_option.scan_parallel = 2
        connection.transaction_option = tx_option
        cursor.execute("select * from tsubakuro_rust_python_test order by pk")
        rows = cursor.fetchall()
        assert rows == [(1,), (2,), (9,)]
        connection.commit()
