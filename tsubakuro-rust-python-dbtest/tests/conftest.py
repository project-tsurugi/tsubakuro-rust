import pytest
import tsurugi_dbapi as tsurugi


def pytest_addoption(parser):
    parser.addoption(
        "--endpoint",
        action="store",
        default="tcp://localhost:12345",
        help="Tsurugi endpoint URL",
    )
    parser.addoption(
        "--blob-relay-service-endpoint",
        action="store",
        help="Blob relay service endpoint URL",
    )
    parser.addoption(
        "--blob-relay-service-ca-cert-pem-file",
        action="store",
        help="Blob relay service CA certificate PEM file",
    )


@pytest.fixture(scope="session")
def endpoint(request):
    return request.config.getoption("--endpoint")


@pytest.fixture(scope="session")
def blob_relay_service_endpoint(request):
    return request.config.getoption("--blob-relay-service-endpoint")


@pytest.fixture(scope="session")
def blob_relay_service_ca_cert_pem_file(request):
    return request.config.getoption("--blob-relay-service-ca-cert-pem-file")


@pytest.fixture(scope="function")
def config(endpoint, blob_relay_service_endpoint, blob_relay_service_ca_cert_pem_file):
    config = tsurugi.Config()
    config.application_name = "tsubakuro-rust-python-dbtest.pytest"
    config.endpoint = endpoint
    config.user = "tsurugi"
    config.password = "password"
    config.session_label = "tsubakuro-rust-python-dbteset.session"
    config.blob_relay_service_endpoint = blob_relay_service_endpoint
    config.blob_relay_service_ca_cert_pem_file = blob_relay_service_ca_cert_pem_file
    return config


@pytest.fixture(scope="function")
def connection(config):
    connection = tsurugi.connect(config)
    yield connection
    connection.close()
