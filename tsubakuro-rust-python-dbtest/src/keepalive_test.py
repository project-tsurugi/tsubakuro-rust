"""Verifying Session Maintenance Behavior Due to TCP Timeouts in the Tsurugi Python DB-API

Test Details:
    1. Create a Connection with default_timeout configured.
    2. Wait after creating the Connection.
    3. After the wait finishes, execute ``SELECT * FROM emp;``.
       If keepalive is working, the execution should succeed.

How to execute:
    uv run src/keepalive_test.py
"""

from __future__ import annotations

import datetime
import logging
import sys
import time
import traceback
from pathlib import Path
from typing import Optional

import tsurugi_dbapi as tsurugi


ENDPOINT = "tcp://localhost:12345"
SQL = "SELECT * from emp;"
APPLICATION_NAME = "tsurugi-keepalive-test"

# tuple(default_timeout [sec], Connection wait[sec])
TEST_CASES: list[tuple[int, int]] = [
    (30, 30),
    (60, 60),
    (120, 120),
    (180, 180),
    (270, 270),
    (300, 300),
    (330, 330),
    (360, 360),
]


def setup_logger(log_path: Path) -> logging.Logger:
    log_path.parent.mkdir(parents=True, exist_ok=True)

    logger = logging.getLogger("keepalive_test")
    logger.setLevel(logging.DEBUG)
    logger.handlers.clear()
    logger.propagate = False

    formatter = logging.Formatter(
        fmt="%(asctime)s [%(levelname)s] %(message)s",
        datefmt="%Y-%m-%d %H:%M:%S",
    )

    file_handler = logging.FileHandler(log_path, encoding="utf-8")
    file_handler.setLevel(logging.DEBUG)
    file_handler.setFormatter(formatter)
    logger.addHandler(file_handler)

    stream_handler = logging.StreamHandler(sys.stdout)
    stream_handler.setLevel(logging.INFO)
    stream_handler.setFormatter(formatter)
    logger.addHandler(stream_handler)

    return logger


def run_case(timeout_sec: int, wait_sec: int, logger: logging.Logger) -> dict:
    result: dict = {
        "timeout_sec": timeout_sec,
        "wait_sec": wait_sec,
        "connect": None,
        "execute": None,
        "fetch": None,
        "commit": None,
        "rowcount": None,
        "error_phase": None,
        "error_type": None,
        "error_message": None,
    }

    logger.info("=" * 80)
    logger.info("run case: default_timeout=%ds, wait=%ds", timeout_sec, wait_sec)

    def record_error(phase: str, exc: BaseException) -> None:
        result["error_phase"] = phase
        result["error_type"] = type(exc).__name__
        result["error_message"] = str(exc)
        logger.error("%s fail: %s: %s", phase, type(exc).__name__, exc)
        logger.debug("%s", traceback.format_exc())

    config = tsurugi.Config(
        endpoint=ENDPOINT,
        application_name=APPLICATION_NAME,
        session_label=f"keepalive timeout={timeout_sec}s wait={wait_sec}s",
        default_timeout=timeout_sec,
    )

    try:
        connection = tsurugi.connect(config)
    except Exception as exc:
        result["connect"] = False
        record_error("connect", exc)
        return result

    result["connect"] = True
    logger.info("connect succeeded")

    try:
        logger.info("Connection created. Wait start: %d[sec]", wait_sec)
        time.sleep(wait_sec)
        logger.info("wait finished. execute SQL: %s", SQL)

        try:
            cursor = connection.cursor()
        except Exception as exc:
            record_error("cursor", exc)
            return result

        try:
            try:
                cursor.execute(SQL)
                result["execute"] = True
                logger.info("execute succeeded")
            except Exception as exc:
                result["execute"] = False
                record_error("execute", exc)
                return result

            try:
                rows = cursor.fetchall()
                result["fetch"] = True
                result["rowcount"] = len(rows)
                logger.info("fetchall succeeded: %d row", len(rows))
            except Exception as exc:
                result["fetch"] = False
                record_error("fetch", exc)
                return result
        finally:
            try:
                cursor.close()
            except Exception as exc:
                logger.warning("cursor.close error: %s: %s", type(exc).__name__, exc)

        try:
            connection.commit()
            result["commit"] = True
            logger.info("commit succeeded")
        except Exception as exc:
            result["commit"] = False
            record_error("commit", exc)
            try:
                connection.rollback()
                logger.info("rollback succeeded")
            except Exception as rb_exc:
                logger.warning("rollback error: %s: %s", type(rb_exc).__name__, rb_exc)

    finally:
        try:
            connection.close()
            logger.info("connection.close succeeded")
        except Exception as exc:
            logger.warning("connection.close error: %s: %s", type(exc).__name__, exc)

    return result


def _cell(value: Optional[bool]) -> str:
    if value is None:
        return "-"
    return "OK" if value else "NG"


def format_summary_table(results: list[dict]) -> str:
    header = (
        f"{'timeout':>8} {'wait':>5} {'connect':>8} {'execute':>8} "
        f"{'fetch':>6} {'commit':>7} {'rows':>6}  {'error_phase':<12} error"
    )
    lines = [header, "-" * len(header)]

    for r in results:
        rows = str(r["rowcount"]) if r["rowcount"] is not None else "-"
        phase = r["error_phase"] or "-"
        if r["error_type"] or r["error_message"]:
            error_msg = f"{r['error_type']}: {r['error_message']}"
        else:
            error_msg = ""
        lines.append(
            f"{r['timeout_sec']:>8} {r['wait_sec']:>5} "
            f"{_cell(r['connect']):>8} {_cell(r['execute']):>8} "
            f"{_cell(r['fetch']):>6} {_cell(r['commit']):>7} {rows:>6}  "
            f"{phase:<12} {error_msg}"
        )

    return "\n".join(lines)


def main() -> int:
    log_dir = Path(__file__).resolve().parent / "logs"
    timestamp = datetime.datetime.now().strftime("%Y%m%d_%H%M%S")
    log_path = log_dir / f"keepalive_{timestamp}.log"

    logger = setup_logger(log_path)
    logger.info("Tsurugi keepalive test start")
    logger.info("endpoint: %s", ENDPOINT)
    logger.info("SQL: %s", SQL)
    logger.info("log file: %s", log_path)
    logger.info(
        "test cases: %s",
        ", ".join(f"(timeout={t}s, wait={w}s)" for t, w in TEST_CASES),
    )

    config = tsurugi.Config(
        endpoint=ENDPOINT,
        application_name=APPLICATION_NAME,
    )
    with tsurugi.connect(config) as connection:
        if connection.find_table_metadata("emp") is None:
            with connection.cursor() as cursor:
                cursor.execute("create table emp (pk int primary key)")
            connection.commit()

    results: list[dict] = []
    for timeout_sec, wait_sec in TEST_CASES:
        result = run_case(timeout_sec, wait_sec, logger)
        results.append(result)

    logger.info("=" * 80)
    logger.info("result summary:\n%s", format_summary_table(results))
    logger.info("Tsurugi keepalive test end")
    return 0


if __name__ == "__main__":
    sys.exit(main())
