import tsurugi_dbapi as tsurugi
from tsurugi_dbapi import ShutdownOption, ShutdownType


def test_config(endpoint):
    config = tsurugi.Config(endpoint=endpoint, user="tsurugi", password="password")
    config.shutdown_option = ShutdownOption(ShutdownType.GRACEFUL, 10)
    with tsurugi.connect(config) as connection:
        with connection.cursor() as cursor:
            cursor.execute("drop table if exists tsubakuro_rust_python_test")
            connection.commit()


def test_connection(connection):
    connection.shutdown_option = ShutdownOption(ShutdownType.GRACEFUL, 10)
    with connection.cursor() as cursor:
        cursor.execute("drop table if exists tsubakuro_rust_python_test")
        connection.commit()


def test_close_NOTHING(connection):
    connection.shutdown_option = ShutdownOption(ShutdownType.NOTHING)
    with connection.cursor() as cursor:
        cursor.execute("drop table if exists tsubakuro_rust_python_test")
        connection.commit()


def test_close_FORCEFUL(connection):
    connection.shutdown_option = ShutdownOption(ShutdownType.FORCEFUL, 10)
    with connection.cursor() as cursor:
        cursor.execute("drop table if exists tsubakuro_rust_python_test")
        connection.commit()


def test_close_GRACEFUL(connection):
    connection.shutdown_option = ShutdownOption(ShutdownType.GRACEFUL, 10)
    with connection.cursor() as cursor:
        cursor.execute("drop table if exists tsubakuro_rust_python_test")
        connection.commit()
