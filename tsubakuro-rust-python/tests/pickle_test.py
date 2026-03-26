import tsurugi_dbapi as tsurugi
import pickle


def test_transaction_type():
    pickle_test_main(tsurugi.TransactionType.OCC)
    pickle_test_main(tsurugi.TransactionType.LTX)
    pickle_test_main(tsurugi.TransactionType.RTX)


def test_transaction_option():
    target = tsurugi.TransactionOption()
    pickle_test_main(target)

    target.transaction_type = tsurugi.TransactionType.LTX
    target.label = "test"
    target.include_ddl = True
    target.write_preserve = ["wp1", "wp2"]
    target.inclusive_read_area = ["ir1", "ir2"]
    target.exclusive_read_area = ["er1", "er2"]
    target.scan_parallel = 44
    target.begin_timeout = 60
    pickle_test_main(target)


def test_commit_type():
    pickle_test_main(tsurugi.CommitType.DEFAULT)
    pickle_test_main(tsurugi.CommitType.ACCEPTED)
    pickle_test_main(tsurugi.CommitType.AVAILABLE)
    pickle_test_main(tsurugi.CommitType.STORED)
    pickle_test_main(tsurugi.CommitType.PROPAGATED)


def test_commit_option():
    target = tsurugi.CommitOption()
    pickle_test_main(target)

    target.commit_type = tsurugi.CommitType.STORED
    target.auto_dispose = True
    target.commit_timeout = 60
    pickle_test_main(target)


def test_shutdown_type():
    pickle_test_main(tsurugi.ShutdownType.NOTHING)
    pickle_test_main(tsurugi.ShutdownType.GRACEFUL)
    pickle_test_main(tsurugi.ShutdownType.FORCEFUL)


def test_shutdown_option():
    target = tsurugi.ShutdownOption()
    pickle_test_main(target)

    target.shutdown_type = tsurugi.ShutdownType.FORCEFUL
    target.shutdown_timeout = 60
    pickle_test_main(target)


def test_config():
    target = tsurugi.Config()
    pickle_test_main(target)

    target.application_name = "test_app"
    target.endpoint = "test_endpoint"
    target.user = "test_user"
    target.password = "test_password"
    target.auth_token = "test_auth_token"
    target.credentials = "test_credentials"
    target.session_label = "test_session"
    target.transaction_option = tsurugi.TransactionOption.ltx(label="test_transaction")
    target.commit_option = tsurugi.CommitOption(tsurugi.CommitType.STORED)
    target.shutdown_option = tsurugi.ShutdownOption(tsurugi.ShutdownType.FORCEFUL)
    target.default_timeout = 60
    pickle_test_main(target)


def pickle_test_main(target):
    dump = pickle.dumps(target)
    load = pickle.loads(dump)
    assert target == load
