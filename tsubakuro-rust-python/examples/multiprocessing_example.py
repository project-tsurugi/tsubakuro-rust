import tsurugi_dbapi as tsurugi
from multiprocessing import Pool


def main():
    config = tsurugi.Config()
    config.application_name = "tsurugi-dbapi multiprocessing example"
    config.endpoint = "tcp://localhost:12345"
    config.user = "tsurugi"
    config.password = "password"
    config.default_timeout = 30

    with Pool(4) as pool:
        pool.map(worker, [config] * 4)


def worker(config):
    with tsurugi.connect(config) as connection:
        tables_names = connection.list_tables()
        print("table_names:", tables_names)


if __name__ == "__main__":
    main()
