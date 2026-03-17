import tsurugi_dbapi as tsurugi


def main():
    # tsurugi.env_logger_init("tsubakuro_rust_python=trace")

    example1()
    example2()
    example3()


def example1():
    print("==== example1 ====")

    config = tsurugi.Config()
    config.application_name = "tsurugi-dbapi example"
    config.endpoint = "tcp://localhost:12345"
    config.user = "tsurugi"
    config.password = "password"
    config.default_timeout = 30  # seconds
    print(config)
    with tsurugi.connect(config) as connection:
        execute(connection)


def example2():
    print("==== example2 ====")

    config = tsurugi.Config(
        application_name="tsurugi-dbapi example",
        endpoint="tcp://localhost:12345",
        user="tsurugi",
        password="password",
        default_timeout=30,  # seconds
    )
    with tsurugi.connect(config) as connection:
        execute(connection)


def example3():
    print("==== example3 ====")

    with tsurugi.connect(
        application_name="tsurugi-dbapi example",
        endpoint="tcp://localhost:12345",
        user="tsurugi",
        password="password",
        default_timeout=30,  # seconds
    ) as connection:
        execute(connection)


def execute(connection):
    print("table_names:", connection.list_tables())

    with connection.cursor() as cursor:
        cursor.execute("drop table if exists tsurugi_dbapi_example")
        cursor.execute(
            "create table tsurugi_dbapi_example (foo int primary key, bar bigint, zzz varchar(10))"
        )
        connection.commit()  # You must commit even with DDL.

        cursor.execute(
            "insert into tsurugi_dbapi_example values (1, 100, 'abc'), (2, 200, 'def'), (3, 300, 'ghi')"
        )
        print("insert rowcount:", cursor.rowcount)
        connection.commit()

        cursor.execute("select * from tsurugi_dbapi_example")
        print("description:", cursor.description)

        for row in cursor:
            print("row:", row)

        connection.commit()


if __name__ == "__main__":
    main()
