import logging
import tsurugi_dbapi as tsurugi


def main():
    # "_tsubakuro_rust_python" means the internal module name.

    logging.basicConfig()
    logger = logging.getLogger("_tsubakuro_rust_python")
    logger.setLevel(5)  # TRACE

    tsurugi.logging_init()

    with tsurugi.connect(
        endpoint="tcp://localhost:12345", user="tsurugi", password="password"
    ) as connection:
        connection.commit()


if __name__ == "__main__":
    main()
