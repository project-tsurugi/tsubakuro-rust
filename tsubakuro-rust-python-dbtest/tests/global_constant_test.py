import tsubakuro_rust_python as tsurugi


def test_global_constants(endpoint):
    assert tsurugi.apilevel == "2.0"
    assert tsurugi.threadsafety == 1
    assert tsurugi.paramstyle == "qmark"
