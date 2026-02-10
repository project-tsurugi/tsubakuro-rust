import tsubakuro_rust_python as tsurugi


def test_connect1(endpoint):
    config = tsurugi.Config()
    config.endpoint = endpoint
    config.user = "tsurugi"
    config.password = "password"
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
