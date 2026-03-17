import pytest
import tsurugi_dbapi as tsurugi


def pytest_addoption(parser):
    parser.addoption(
        "--endpoint",
        action="store",
        default="tcp://localhost:12345",
        help="Tsurugi endpoint URL",
    )


@pytest.fixture(scope="session")
def endpoint(request):
    return request.config.getoption("--endpoint")


@pytest.fixture(scope="function")
def connection(endpoint):
    connection = tsurugi.connect(endpoint=endpoint, user="tsurugi", password="password")
    yield connection
    connection.close()
