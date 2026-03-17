from tsurugi_dbapi import CommitOption, CommitType


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
