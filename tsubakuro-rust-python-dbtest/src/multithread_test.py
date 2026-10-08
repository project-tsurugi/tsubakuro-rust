"""Multithread Test for Tsurugi Python DB-API

How to execute:
    uv run src/multithread_test.py
"""

import logging
import sys
import time

import tsurugi_dbapi as tsurugi


ENDPOINT = "tcp://localhost:12345"
APPLICATION_NAME = "tsubakuro-rust-python-multithread-test"


def prepare():
    config = tsurugi.Config(
        endpoint=ENDPOINT,
        application_name=APPLICATION_NAME,
    )
    with tsurugi.connect(config) as connection, connection.cursor() as cursor:
        cursor.execute("drop table if exists test")
        cursor.execute("create table test (pk int primary key)")
        connection.commit()

        sql = "insert into test (pk) values "
        for i in range(60):
            if i > 0:
                sql += ", "
            sql += f"({i})"
        cursor.execute(sql)
        connection.commit()


def execute(number: int):
    logger = logging.getLogger("multithread_test")
    logger.setLevel(logging.INFO)

    config = tsurugi.Config(
        endpoint=ENDPOINT,
        application_name=APPLICATION_NAME,
    )

    with tsurugi.connect(config) as connection, connection.cursor() as cursor:
        start_time = time.time()
        logger.info("execute%d start", number)

        cursor.execute("select * from test, test, test")
        cursor.fetchall()

        end_time = time.time()
        logger.info(
            "execute%d end (elapsed: %.2f sec)",
            number,
            end_time - start_time,
        )
        connection.commit()


def main() -> int:
    logging.basicConfig(
        level=logging.INFO,
        format="%(asctime)s.%(msecs)03d [%(levelname)s] %(message)s",
        datefmt="%Y-%m-%d %H:%M:%S",
    )
    logger = logging.getLogger("multithread_test")
    logger.setLevel(logging.INFO)

    logger.info("prepare start")
    prepare()
    logger.info("prepare end")

    from concurrent.futures import ThreadPoolExecutor

    with ThreadPoolExecutor(max_workers=10) as executor:
        logger.info("multithread execution start")
        start_time = time.time()
        futures = [executor.submit(execute, i) for i in range(10)]
        for future in futures:
            future.result()
        end_time = time.time()
        logger.info(
            "multithread execution end (elapsed: %.2f sec)", end_time - start_time
        )
    return 0


if __name__ == "__main__":
    sys.exit(main())
