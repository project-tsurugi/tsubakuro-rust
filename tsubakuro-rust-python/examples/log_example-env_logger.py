import tsurugi_dbapi as tsurugi


def main():
    # "tsubakuro_rust_python" means the internal module name.

    # tsurugi.env_logger_init("trace")
    tsurugi.env_logger_init("tsubakuro_rust_python=trace")
    # tsurugi.env_logger_init()  # same as "tsubakuro_rust_python=info"
    # tsurugi.env_logger_init("tsubakuro_rust_python=trace", "/tmp/tsubakuro-rust-python.log")

    with tsurugi.connect(
        endpoint="tcp://localhost:12345", user="tsurugi", password="password"
    ) as connection:
        connection.commit()


if __name__ == "__main__":
    main()
