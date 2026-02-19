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


def test_transaction_option_occ():
    tx1_option = TransactionOption(TransactionType.OCC)
    tx1_option.label = "OCC transaction"

    tx2_option = TransactionOption.occ(label="OCC transaction")

    assert_compare(tx1_option, tx2_option)


def test_transaction_option_ltx():
    tx1_option = TransactionOption(TransactionType.LTX)
    tx1_option.label = "LTX transaction"
    tx1_option.write_preserve = ["table1", "table2"]
    tx1_option.inclusive_read_area = ["table3", "table4"]
    tx1_option.exclusive_read_area = ["table5", "table6"]

    tx2_option = TransactionOption.ltx(
        label="LTX transaction",
        write_preserve=["table1", "table2"],
        inclusive_read_area=["table3", "table4"],
        exclusive_read_area=["table5", "table6"],
    )

    assert_compare(tx1_option, tx2_option)


def test_transaction_option_ddl():
    tx1_option = TransactionOption(TransactionType.LTX)
    tx1_option.label = "LTX transaction for DDL"
    tx1_option.include_ddl = True

    tx2_option = TransactionOption.ddl(label="LTX transaction for DDL")

    assert_compare(tx1_option, tx2_option)


def test_transaction_option_rtx():
    tx1_option = TransactionOption(TransactionType.RTX)
    tx1_option.label = "RTX transaction"
    tx1_option.scan_parallel = 4

    tx2_option = TransactionOption.rtx(label="RTX transaction", scan_parallel=4)

    assert_compare(tx1_option, tx2_option)


def assert_compare(tx1_option, tx2_option):
    assert tx1_option.transaction_type == tx2_option.transaction_type
    assert tx1_option.label == tx2_option.label
    assert tx1_option.include_ddl == tx2_option.include_ddl
    assert tx1_option.write_preserve == tx2_option.write_preserve
    assert tx1_option.inclusive_read_area == tx2_option.inclusive_read_area
    assert tx1_option.exclusive_read_area == tx2_option.exclusive_read_area
    assert tx1_option.scan_parallel == tx2_option.scan_parallel
    assert tx1_option.begin_timeout == tx2_option.begin_timeout
