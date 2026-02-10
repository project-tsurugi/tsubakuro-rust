import tsubakuro_rust_python as tsurugi
from tsubakuro_rust_python import ShutdownOption, ShutdownType


def test_shutdown_option():
    option = ShutdownOption()
    assert option.shutdown_type == ShutdownType.GRACEFUL
    assert option.shutdown_timeout is None

    option = ShutdownOption(ShutdownType.NOTHING)
    assert option.shutdown_type == ShutdownType.NOTHING
    assert option.shutdown_timeout is None

    option = ShutdownOption(ShutdownType.FORCEFUL, 30)
    assert option.shutdown_type == ShutdownType.FORCEFUL
    assert option.shutdown_timeout == 30


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
