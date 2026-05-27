import tsurugi_dbapi as tsurugi


def main():
    config = tsurugi.Config()
    config.application_name = "tsurugi-dbapi example"
    config.endpoint = "tcp://localhost:12345"
    config.user = "tsurugi"
    config.password = "password"
    config.default_timeout = 30  # seconds
    with tsurugi.connect(config) as connection:
        print("lob_transfer_type:", connection.lob_transfer_type())

        execute_blob(connection)
        execute_upload(connection)
        execute_prepare(connection)


def create_table(connection):
    with connection.cursor() as cursor:
        cursor.execute("drop table if exists blob_example")
        cursor.execute("create table blob_example (pk int primary key, value blob)")
        connection.commit()  # You must commit even with DDL.


def execute_blob(connection):
    print("=== execute_blob() ===")
    create_table(connection)

    with connection.cursor() as cursor:
        insert_sql = "insert into blob_example values (?, ?)"
        parameters_list = [
            (1, tsurugi.type_code.Blob(None)),
            (2, tsurugi.type_code.Blob(b"abc")),
            (3, tsurugi.type_code.Blob(b"def")),
        ]
        cursor.executemany(insert_sql, parameters_list)
        print("insert rowcount:", cursor.rowcount)
        connection.commit()

        select(connection)


def execute_upload(connection):
    print("=== execute_upload() ===")
    create_table(connection)

    with connection.cursor() as cursor:
        insert_sql = "insert into blob_example values (?, ?)"
        parameters_list = [
            (1, cursor.upload_blob(None)),
            (2, cursor.upload_blob(b"abc")),
            (3, cursor.upload_blob(b"def")),
        ]
        cursor.executemany(insert_sql, parameters_list)
        print("insert rowcount:", cursor.rowcount)
        connection.commit()

        select(connection)


def execute_prepare(connection):
    print("=== execute_prepare() ===")
    create_table(connection)

    with connection.cursor() as cursor:
        insert_sql = "insert into blob_example values (?, ?)"
        placeholder_list = [tsurugi.type_code.Int32, tsurugi.type_code.Blob]
        cursor.prepare(insert_sql, placeholder_list)

        parameters_list = [
            (1, None),
            (2, b"abc"),
            (3, b"def"),
        ]
        cursor.executemany(insert_sql, parameters_list)
        print("insert rowcount:", cursor.rowcount)
        connection.commit()

        select(connection)


def select(connection):
    with connection.cursor() as cursor:
        select_sql = "select * from blob_example order by pk"
        cursor.execute(select_sql)
        for row in cursor:
            print("row:", row)

        connection.commit()


if __name__ == "__main__":
    main()
