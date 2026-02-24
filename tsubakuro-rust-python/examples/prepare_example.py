import tsubakuro_rust_python as tsurugi


def main():
    config = tsurugi.Config()
    config.application_name = "tsubakuro-rust-python example"
    config.endpoint = "tcp://localhost:12345"
    config.user = "tsurugi"
    config.password = "password"
    config.default_timeout = 30  # seconds
    with tsurugi.connect(config) as connection:
        execute_qmark(connection)
        prepare_qmark(connection)
        execute_named(connection)
        prepare_named(connection)


def execute_qmark(connection):
    with connection.cursor() as cursor:
        cursor.execute("drop table if exists tsubakuro_rust_python_example")
        cursor.execute(
            "create table tsubakuro_rust_python_example (foo int primary key, bar bigint, zzz varchar(10))"
        )
        connection.commit()  # You must commit even with DDL.

        insert_sql = "insert into tsubakuro_rust_python_example values (?, ?, ?)"
        # To distinguish between Int32 and Int64, you must specify the type at least in the first parameters.
        parameters_list = [
            (
                tsurugi.type_code.Int32(1),
                tsurugi.type_code.Int64(100),
                tsurugi.type_code.Str("abc"),
            ),
            (2, 200, "def"),
            (3, 300, "ghi"),
        ]
        cursor.executemany(insert_sql, parameters_list)
        print("insert rowcount:", cursor.rowcount)
        connection.commit()

        select_sql = "select * from tsubakuro_rust_python_example where foo = ?"
        cursor.execute(select_sql, (tsurugi.type_code.Int32(2),))
        row = cursor.fetchone()
        print("row:", row)

        connection.commit()

        cursor.clear()  # When explicitly clearing cached prepared statements.


# use cursor.prepare() (tsubakuro-rust-python's proprietary specifications)
def prepare_qmark(connection):
    with connection.cursor() as cursor:
        cursor.execute("drop table if exists tsubakuro_rust_python_example")
        cursor.execute(
            "create table tsubakuro_rust_python_example (foo int primary key, bar bigint, zzz varchar(10))"
        )
        connection.commit()  # You must commit even with DDL.

        insert_sql = "insert into tsubakuro_rust_python_example values (?, ?, ?)"
        cursor.prepare(
            insert_sql,
            (tsurugi.type_code.Int32, tsurugi.type_code.Int64, tsurugi.type_code.Str),
        )
        cursor.executemany(
            insert_sql, [(1, 100, "abc"), (2, 200, "def"), (3, 300, "ghi")]
        )
        print("insert rowcount:", cursor.rowcount)
        connection.commit()

        select_sql = "select * from tsubakuro_rust_python_example where foo = ?"
        cursor.prepare(select_sql, (tsurugi.type_code.Int32,))
        cursor.execute(select_sql, (2,))
        row = cursor.fetchone()
        print("row:", row)

        connection.commit()

        cursor.clear()  # When explicitly clearing cached prepared statements.


def execute_named(connection):
    with connection.cursor() as cursor:
        cursor.execute("drop table if exists tsubakuro_rust_python_example")
        cursor.execute(
            "create table tsubakuro_rust_python_example (foo int primary key, bar bigint, zzz varchar(10))"
        )
        connection.commit()  # You must commit even with DDL.

        insert_sql = (
            "insert into tsubakuro_rust_python_example values (:foo, :bar, :zzz)"
        )
        # To distinguish between Int32 and Int64, you must specify the type at least in the first parameters.
        parameters_list = [
            {
                "foo": tsurugi.type_code.Int32(1),
                "bar": tsurugi.type_code.Int64(100),
                "zzz": tsurugi.type_code.Str("abc"),
            },
            {"foo": 2, "bar": 200, "zzz": "def"},
            {"foo": 3, "bar": 300, "zzz": "ghi"},
        ]
        cursor.executemany(insert_sql, parameters_list)
        print("insert rowcount:", cursor.rowcount)
        connection.commit()

        select_sql = "select * from tsubakuro_rust_python_example where foo = :foo"
        cursor.execute(select_sql, {"foo": tsurugi.type_code.Int32(2)})
        row = cursor.fetchone()
        print("row:", row)

        connection.commit()

        cursor.clear()  # When explicitly clearing cached prepared statements.


# use cursor.prepare() (tsubakuro-rust-python's proprietary specifications)
def prepare_named(connection):
    with connection.cursor() as cursor:
        cursor.execute("drop table if exists tsubakuro_rust_python_example")
        cursor.execute(
            "create table tsubakuro_rust_python_example (foo int primary key, bar bigint, zzz varchar(10))"
        )
        connection.commit()  # You must commit even with DDL.

        insert_sql = (
            "insert into tsubakuro_rust_python_example values (:foo, :bar, :zzz)"
        )
        cursor.prepare(
            insert_sql,
            {
                "foo": tsurugi.type_code.Int32,
                "bar": tsurugi.type_code.Int64,
                "zzz": tsurugi.type_code.Str,
            },
        )
        cursor.executemany(
            insert_sql,
            [
                {"foo": 1, "bar": 100, "zzz": "abc"},
                {"foo": 2, "bar": 200, "zzz": "def"},
                {"foo": 3, "bar": 300, "zzz": "ghi"},
            ],
        )
        print("insert rowcount:", cursor.rowcount)
        connection.commit()

        select_sql = "select * from tsubakuro_rust_python_example where foo = :foo"
        cursor.prepare(select_sql, {"foo": tsurugi.type_code.Int32})
        cursor.execute(select_sql, {"foo": 2})
        row = cursor.fetchone()
        print("row:", row)

        connection.commit()

        cursor.clear()  # When explicitly clearing cached prepared statements.


if __name__ == "__main__":
    main()
