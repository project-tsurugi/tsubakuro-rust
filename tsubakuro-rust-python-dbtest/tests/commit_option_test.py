import tsurugi_dbapi as tsurugi
from tsurugi_dbapi import CommitOption, CommitType


def test_config(endpoint):
    config = tsurugi.Config(endpoint=endpoint, user="tsurugi", password="password")
    config.commit_option = CommitOption(CommitType.STORED, True)
    with tsurugi.connect(config) as connection:
        with connection.cursor() as cursor:
            cursor.execute("drop table if exists tsubakuro_rust_python_test")
            connection.commit()


def test_connection(connection):
    connection.commit_option = CommitOption(CommitType.STORED, True)
    with connection.cursor() as cursor:
        cursor.execute("drop table if exists tsubakuro_rust_python_test")
        connection.commit()


def test_commit(connection):
    with connection.cursor() as cursor:
        cursor.execute("drop table if exists tsubakuro_rust_python_test")
        connection.commit(CommitOption(CommitType.STORED, True, 60))
