import tsubakuro_rust_python as tsurugi


def test_config():
    tx1_option = tsurugi.TransactionOption(tsurugi.TransactionType.LTX)
    tx1_option.label = "ltx"
    commit_option1 = tsurugi.CommitOption(tsurugi.CommitType.STORED, True, 20)
    shutdown_option1 = tsurugi.ShutdownOption(tsurugi.ShutdownType.FORCEFUL, 10)
    config1 = tsurugi.Config()
    config1.application_name = "app"
    config1.endpoint = "http://localhost:8080"
    config1.user = "tsurugi"
    config1.password = "password"
    config1.auth_token = "token"
    config1.credentials = "credentials"
    config1.session_label = "session"
    config1.transaction_option = tx1_option
    config1.commit_option = commit_option1
    config1.shutdown_option = shutdown_option1
    config1.default_timeout = 30

    tx2_option = tsurugi.TransactionOption(tsurugi.TransactionType.LTX)
    tx2_option.label = "ltx"
    commit_option2 = tsurugi.CommitOption(tsurugi.CommitType.STORED, True, 20)
    shutdown_option2 = tsurugi.ShutdownOption(tsurugi.ShutdownType.FORCEFUL, 10)
    config2 = tsurugi.Config(
        application_name="app",
        endpoint="http://localhost:8080",
        user="tsurugi",
        password="password",
        auth_token="token",
        credentials="credentials",
        session_label="session",
        transaction_option=tx2_option,
        commit_option=commit_option2,
        shutdown_option=shutdown_option2,
        default_timeout=30,
    )

    tx3_option = tsurugi.TransactionOption(tsurugi.TransactionType.LTX)
    tx3_option.label = "ltx"
    commit_option3 = tsurugi.CommitOption(tsurugi.CommitType.STORED, True, 20)
    shutdown_option3 = tsurugi.ShutdownOption(tsurugi.ShutdownType.FORCEFUL, 10)
    config3 = tsurugi.Config(
        tx3_option,
        commit_option3,
        shutdown_option3,
        application_name="app",
        endpoint="http://localhost:8080",
        user="tsurugi",
        password="password",
        auth_token="token",
        credentials="credentials",
        session_label="session",
        default_timeout=30,
    )

    assert_compare(config1, config2)
    assert_compare(config1, config3)


def assert_compare(config1, config2):
    assert config1.application_name == config2.application_name
    assert config1.endpoint == config2.endpoint
    assert config1.user == config2.user
    assert config1.password == config2.password
    assert config1.auth_token == config2.auth_token
    assert config1.credentials == config2.credentials
    assert config1.session_label == config2.session_label
    assert (
        config1.transaction_option.transaction_type
        == config2.transaction_option.transaction_type
    )
    assert config1.transaction_option.label == config2.transaction_option.label
    assert config1.commit_option.commit_type == config2.commit_option.commit_type
    assert (
        config1.shutdown_option.shutdown_type == config2.shutdown_option.shutdown_type
    )
    assert config1.default_timeout == config2.default_timeout
