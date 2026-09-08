import tsurugi_dbapi as tsurugi


def main():
    config = tsurugi.Config()
    config.application_name = "tsurugi-dbapi example"
    config.endpoint = "tcp://localhost:12345"
    config.user = "tsurugi"
    config.password = "password"
    config.default_timeout = 30  # seconds
    with tsurugi.connect(config) as connection:
        create_bigint_table(connection)

        explain(connection)
        explain_qmark(connection)
        explain_named(connection)


def create_bigint_table(connection):
    with connection.cursor() as cursor:
        cursor.execute("drop table if exists tsurugi_dbapi_example")
        cursor.execute(
            "create table tsurugi_dbapi_example (foo bigint primary key, bar double, zzz varchar(10))"
        )
        connection.commit()  # You must commit even with DDL.


def explain(connection):
    with connection.cursor() as cursor:
        select_sql = "select * from tsurugi_dbapi_example"
        explain_result = cursor.explain(select_sql)
        print_explain_result(select_sql, explain_result)


def explain_qmark(connection):
    with connection.cursor() as cursor:
        insert_sql = "insert into tsurugi_dbapi_example values (?, ?, ?)"
        # Python's int is treated as a BIGINT, float as a DOUBLE, and str as a CHAR or VARCHAR.
        parameters = (1, 1.5, "abc")
        explain_result = cursor.explain(insert_sql, parameters)
        print_explain_result("insert explain", explain_result)

        select_sql = "select * from tsurugi_dbapi_example where foo = ?"
        explain_result = cursor.explain(select_sql, (1,))
        print_explain_result("select explain", explain_result)


def explain_named(connection):
    with connection.cursor() as cursor:
        insert_sql = "insert into tsurugi_dbapi_example values (:foo, :bar, :zzz)"
        # Python's int is treated as a BIGINT, float as a DOUBLE, and str as a CHAR or VARCHAR.
        parameters = {"foo": 2, "bar": 2.5, "zzz": "def"}
        explain_result = cursor.explain(insert_sql, parameters)
        print_explain_result("insert explain", explain_result)

        select_sql = "select * from tsurugi_dbapi_example where foo = :foo"
        explain_result = cursor.explain(select_sql, {"foo": 2})
        print_explain_result("select explain", explain_result)


def print_explain_result(title, explain_result):
    print("====", title, "====")
    print("format_id:", explain_result.format_id)
    print("format_version:", explain_result.format_version)
    print("contents:", explain_result.contents)
    print("columns:")
    for column in explain_result.columns:
        print("  name:", column.name, ", type_code:", column.type_code)


if __name__ == "__main__":
    main()
