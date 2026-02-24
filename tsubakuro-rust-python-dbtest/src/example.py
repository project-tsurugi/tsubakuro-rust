import tsubakuro_rust_python as tsurugi


def main():
    # tsurugi.env_logger_init("tsubakuro_rust_python=trace")

    example1()
    example2()
    example3()


def example1():
    config = tsurugi.Config()
    config.application_name = "tsubakuro-rust-python example"
    config.endpoint = "tcp://localhost:12345"
    config.user = "tsurugi"
    config.password = "password"
    config.default_timeout = 30  # seconds
    print(config)
    with tsurugi.connect(config) as connection:
        print("table_names:", connection.list_tables())
        execute(connection)


def example2():
    config = tsurugi.Config(
        application_name="tsubakuro-rust-python example",
        endpoint="tcp://localhost:12345",
        user="tsurugi",
        password="password",
        default_timeout=30,  # seconds
    )
    with tsurugi.connect(config) as connection:
        print("table_names:", connection.list_tables())


def example3():
    with tsurugi.connect(
        application_name="tsubakuro-rust-python example",
        endpoint="tcp://localhost:12345",
        user="tsurugi",
        password="password",
        default_timeout=30,  # seconds
    ) as connection:
        print("table_names:", connection.list_tables())


def execute(connection):
    with connection.cursor() as cursor:
        cursor.execute("drop table if exists tsubakuro_rust_python_example")
        cursor.execute(
            "create table tsubakuro_rust_python_example (foo int primary key, bar bigint, zzz varchar(10))"
        )
        connection.commit()

        cursor.execute(
            "insert into tsubakuro_rust_python_example values (1, 100, 'abc'), (2, 200, 'def'), (3, 300, 'ghi')"
        )
        print("insert rowcount:", cursor.rowcount)
        connection.commit()

        cursor.execute("select * from tsubakuro_rust_python_example")
        print("description:", cursor.description)
        while True:
            row = cursor.fetchone()
            if row is None:
                break

            print("row:", row)
        connection.commit()


if __name__ == "__main__":
    main()
