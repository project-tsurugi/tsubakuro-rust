import tsurugi_dbapi as tsurugi


def test_cc_exception(endpoint):
    with tsurugi.connect(
        endpoint=endpoint, user="tsurugi", password="password"
    ) as connection:
        with connection.cursor() as cursor:
            cursor.execute("drop table if exists tsubakuro_rust_python_test")
            cursor.execute(
                "create table tsubakuro_rust_python_test (foo int primary key, bar int, zzz varchar(10))"
            )
            connection.commit()

    tx1_option = tsurugi.TransactionOption.ltx()
    tx1_option.write_preserve = ["tsubakuro_rust_python_test"]
    config1 = tsurugi.Config(
        endpoint=endpoint,
        user="tsurugi",
        password="password",
        transaction_option=tx1_option,
    )

    tx2_option = tsurugi.TransactionOption.occ()
    config2 = tsurugi.Config(
        endpoint=endpoint,
        user="tsurugi",
        password="password",
        transaction_option=tx2_option,
    )

    with tsurugi.connect(config1) as connection1:
        with tsurugi.connect(config2) as connection2:
            with connection1.cursor() as cursor1:
                with connection2.cursor() as cursor2:
                    cc_exception_test_main(cursor1, cursor2)


def cc_exception_test_main(cursor1, cursor2):
    connection1 = cursor1.connection
    connection2 = cursor2.connection

    # LTX
    cursor1.execute("insert into tsubakuro_rust_python_test values (1, 100, 'abc')")

    # OCC
    try:
        cursor2.execute("insert into tsubakuro_rust_python_test values (2, 200, 'def')")
        connection2.commit()
        assert False, "Expected CcException was not raised"
    except tsurugi.error.CcException as e:
        assert isinstance(e, tsurugi.error.ConflictOnWritePreserveException), (
            f"Expected ConflictOnWritePreserveException, got {type(e)}"
        )

    connection1.commit()

    # OCC
    connection2.rollback()  # Resolve INACTIVE_TRANSACTION
    cursor2.execute("insert into tsubakuro_rust_python_test values (2, 200, 'def')")
    connection2.commit()
