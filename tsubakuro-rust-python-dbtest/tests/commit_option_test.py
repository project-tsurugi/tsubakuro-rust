import tsubakuro_rust_python as tsurugi
from tsubakuro_rust_python import CommitOption, CommitType


def test_commit_option():
    option = CommitOption()
    assert option.commit_type == CommitType.DEFAULT
    assert option.auto_dispose is False
    assert option.commit_timeout is None

    option = CommitOption(CommitType.STORED)
    assert option.commit_type == CommitType.STORED
    assert option.auto_dispose is False
    assert option.commit_timeout is None

    option = CommitOption(CommitType.STORED, True)
    assert option.commit_type == CommitType.STORED
    assert option.auto_dispose is True
    assert option.commit_timeout is None

    option = CommitOption(CommitType.STORED, True, 30)
    assert option.commit_type == CommitType.STORED
    assert option.auto_dispose is True
    assert option.commit_timeout == 30


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
