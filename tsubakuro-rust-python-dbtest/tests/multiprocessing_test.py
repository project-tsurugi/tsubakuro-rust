import tsurugi_dbapi as tsurugi
import multiprocessing


def test_multiprocessing(config):
    p = multiprocessing.Process(target=worker, args=(config,))
    p.start()
    p.join()
    assert p.exitcode == 0


def worker(config):
    with tsurugi.connect(config) as connection:
        with connection.cursor() as cursor:
            cursor.execute("drop table if exists tsubakuro_rust_python_test")
            connection.commit()
