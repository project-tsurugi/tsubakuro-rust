"""
Python library for Tsurugi.

Examples:

    ```python
    import tsubakuro_rust_python as tsurugi

    config = tsurugi.Config()
    config.endpoint = "tcp://localhost:12345"
    config.user = "tsurugi"
    config.password = "password"
    config.default_timeout = 30  # seconds
    with tsurugi.connect(config) as connection:
        with connection.cursor() as cursor:
            cursor.execute("insert into example values (1, 100, 'abc')")
            print("insert rowcount:", cursor.rowcount)
            connection.commit()

            cursor.execute("select * from example")
            for row in cursor:
                print("row:", row)
            connection.commit()
    ```

Note:
    See `Config`, `connect()`, `Connection`, and `Cursor` for more details.
"""

from ._tsubakuro_rust_python import *

__all__ = [name for name in globals() if not name.startswith("_")]
