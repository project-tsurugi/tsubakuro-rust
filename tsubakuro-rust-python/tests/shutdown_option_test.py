from tsurugi_dbapi import ShutdownOption, ShutdownType


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
