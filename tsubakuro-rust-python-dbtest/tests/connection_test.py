import pytest
import tsurugi_dbapi as tsurugi


def test_connect(endpoint):
    config = tsurugi.Config()
    config.application_name = "tsubakuro-rust-python-dbtest.pytest"
    config.endpoint = endpoint
    config.user = "tsurugi"
    config.password = "password"
    config.session_label = "tsubakuro-rust-python-dbteset.session"
    with tsurugi.connect(config) as connection:
        assert isinstance(connection.list_tables(), list)


def test_connect2(endpoint):
    config = tsurugi.Config(endpoint=endpoint, user="tsurugi", password="password")
    with tsurugi.connect(config) as connection:
        assert isinstance(connection.list_tables(), list)


def test_connect3(endpoint):
    with tsurugi.connect(
        endpoint=endpoint, user="tsurugi", password="password"
    ) as connection:
        assert isinstance(connection.list_tables(), list)


def test_closed(connection):
    assert connection.closed is False
    connection.close()
    assert connection.closed is True

    try:
        connection.cursor()
    except tsurugi.ProgrammingError as e:
        assert str(e) == "Connection is already closed"

    try:
        connection.commit()
    except tsurugi.ProgrammingError as e:
        assert str(e) == "Connection is already closed"

    try:
        connection.rollback()
    except tsurugi.ProgrammingError as e:
        assert str(e) == "Connection is already closed"


def test_with_exception(endpoint):
    with pytest.raises(Exception, match="Test exception in with block"):
        with tsurugi.connect(endpoint=endpoint, user="tsurugi", password="password"):
            raise Exception("Test exception in with block")
