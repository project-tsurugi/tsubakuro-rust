import tsurugi_dbapi as tsurugi


def main():
    config = tsurugi.Config()
    config.application_name = "tsurugi-dbapi example"
    config.endpoint = "tcp://localhost:12345"
    config.user = "tsurugi"
    config.password = "password"
    config.default_timeout = 30  # seconds
    with tsurugi.connect(config) as connection:
        execute_qmark(connection)
        execute_named(connection)

        execute_type_code_qmark(connection)
        prepare_type_code_qmark(connection)
        execute_type_code_named(connection)
        prepare_type_code_named(connection)


def create_bigint_table(connection):
    with connection.cursor() as cursor:
        cursor.execute("drop table if exists tsurugi_dbapi_example")
        cursor.execute(
            "create table tsurugi_dbapi_example (foo bigint primary key, bar double, zzz varchar(10))"
        )
        connection.commit()  # You must commit even with DDL.


def execute_qmark(connection):
    create_bigint_table(connection)

    with connection.cursor() as cursor:
        insert_sql = "insert into tsurugi_dbapi_example values (?, ?, ?)"
        # Python's int is treated as a BIGINT, float as a DOUBLE, and str as a CHAR or VARCHAR.
        parameters_list = [
            (1, 1.5, "abc"),
            (2, 2.5, "def"),
            (3, 3.5, "ghi"),
        ]
        cursor.executemany(insert_sql, parameters_list)
        print("insert rowcount:", cursor.rowcount)
        connection.commit()

        select_sql = "select * from tsurugi_dbapi_example where foo = ?"
        cursor.execute(select_sql, (2,))
        row = cursor.fetchone()
        print("row:", row)

        connection.commit()

        cursor.clear()  # When explicitly clearing cached prepared statements.


def execute_named(connection):
    create_bigint_table(connection)

    with connection.cursor() as cursor:
        insert_sql = "insert into tsurugi_dbapi_example values (:foo, :bar, :zzz)"
        # Python's int is treated as a BIGINT, float as a DOUBLE, and str as a CHAR or VARCHAR.
        parameters_list = [
            {"foo": 1, "bar": 1.5, "zzz": "abc"},
            {"foo": 2, "bar": 2.5, "zzz": "def"},
            {"foo": 3, "bar": 3.5, "zzz": "ghi"},
        ]
        cursor.executemany(insert_sql, parameters_list)
        print("insert rowcount:", cursor.rowcount)
        connection.commit()

        select_sql = "select * from tsurugi_dbapi_example where foo = :foo"
        cursor.execute(select_sql, {"foo": 2})
        row = cursor.fetchone()
        print("row:", row)

        connection.commit()

        cursor.clear()  # When explicitly clearing cached prepared statements.


def create_int_table(connection):
    with connection.cursor() as cursor:
        cursor.execute("drop table if exists tsurugi_dbapi_example")
        cursor.execute(
            "create table tsurugi_dbapi_example (foo int primary key, bar bigint, zzz decimal(5))"
        )
        connection.commit()  # You must commit even with DDL.


def execute_type_code_qmark(connection):
    create_int_table(connection)

    with connection.cursor() as cursor:
        insert_sql = "insert into tsurugi_dbapi_example values (?, ?, ?)"
        # To distinguish between Int32, Int64, and Decimal, you must specify the type at least in the first parameters.
        parameters_list = [
            (
                tsurugi.type_code.Int32(1),
                tsurugi.type_code.Int64(100),
                tsurugi.type_code.Decimal(1000),
            ),
            (2, 200, 2000),
            (3, 300, 3000),
        ]
        cursor.executemany(insert_sql, parameters_list)
        print("insert rowcount:", cursor.rowcount)
        connection.commit()

        select_sql = "select * from tsurugi_dbapi_example where foo = ?"
        cursor.execute(select_sql, (tsurugi.type_code.Int32(2),))
        row = cursor.fetchone()
        print("row:", row)

        connection.commit()

        cursor.clear()  # When explicitly clearing cached prepared statements.


# use cursor.prepare() (tsurugi-dbapi's proprietary specifications)
def prepare_type_code_qmark(connection):
    create_int_table(connection)

    with connection.cursor() as cursor:
        insert_sql = "insert into tsurugi_dbapi_example values (?, ?, ?)"
        cursor.prepare(
            insert_sql,
            (
                tsurugi.type_code.Int32,
                tsurugi.type_code.Int64,
                tsurugi.type_code.Decimal,
            ),
        )
        cursor.executemany(insert_sql, [(1, 100, 1000), (2, 200, 2000), (3, 300, 3000)])
        print("insert rowcount:", cursor.rowcount)
        connection.commit()

        select_sql = "select * from tsurugi_dbapi_example where foo = ?"
        cursor.prepare(select_sql, (tsurugi.type_code.Int32,))
        cursor.execute(select_sql, (2,))
        row = cursor.fetchone()
        print("row:", row)

        connection.commit()

        cursor.clear()  # When explicitly clearing cached prepared statements.


def execute_type_code_named(connection):
    create_int_table(connection)

    with connection.cursor() as cursor:
        insert_sql = "insert into tsurugi_dbapi_example values (:foo, :bar, :zzz)"
        # To distinguish between Int32, Int64, and Decimal, you must specify the type at least in the first parameters.
        parameters_list = [
            {
                "foo": tsurugi.type_code.Int32(1),
                "bar": tsurugi.type_code.Int64(100),
                "zzz": tsurugi.type_code.Decimal(1000),
            },
            {"foo": 2, "bar": 200, "zzz": 2000},
            {"foo": 3, "bar": 300, "zzz": 3000},
        ]
        cursor.executemany(insert_sql, parameters_list)
        print("insert rowcount:", cursor.rowcount)
        connection.commit()

        select_sql = "select * from tsurugi_dbapi_example where foo = :foo"
        cursor.execute(select_sql, {"foo": tsurugi.type_code.Int32(2)})
        row = cursor.fetchone()
        print("row:", row)

        connection.commit()

        cursor.clear()  # When explicitly clearing cached prepared statements.


# use cursor.prepare() (tsurugi-dbapi's proprietary specifications)
def prepare_type_code_named(connection):
    create_int_table(connection)

    with connection.cursor() as cursor:
        insert_sql = "insert into tsurugi_dbapi_example values (:foo, :bar, :zzz)"
        cursor.prepare(
            insert_sql,
            {
                "foo": tsurugi.type_code.Int32,
                "bar": tsurugi.type_code.Int64,
                "zzz": tsurugi.type_code.Decimal,
            },
        )
        cursor.executemany(
            insert_sql,
            [
                {"foo": 1, "bar": 100, "zzz": 1000},
                {"foo": 2, "bar": 200, "zzz": 2000},
                {"foo": 3, "bar": 300, "zzz": 3000},
            ],
        )
        print("insert rowcount:", cursor.rowcount)
        connection.commit()

        select_sql = "select * from tsurugi_dbapi_example where foo = :foo"
        cursor.prepare(select_sql, {"foo": tsurugi.type_code.Int32})
        cursor.execute(select_sql, {"foo": 2})
        row = cursor.fetchone()
        print("row:", row)

        connection.commit()

        cursor.clear()  # When explicitly clearing cached prepared statements.


if __name__ == "__main__":
    main()
