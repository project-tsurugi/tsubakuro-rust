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


def test_occ():
    tx1_option = TransactionOption(TransactionType.OCC)
    tx1_option.label = "OCC transaction"

    tx2_option = TransactionOption.occ(label="OCC transaction")

    assert_compare(tx1_option, tx2_option)


def test_ltx():
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


def test_ddl():
    tx1_option = TransactionOption(TransactionType.LTX)
    tx1_option.label = "LTX transaction for DDL"
    tx1_option.include_ddl = True

    tx2_option = TransactionOption.ddl(label="LTX transaction for DDL")

    assert_compare(tx1_option, tx2_option)


def test_rtx():
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
