"""
Tsurugi Python DB-API.

Examples:

    ```python
    import tsurugi_dbapi as tsurugi

    with tsurugi.connect(
        endpoint="tcp://localhost:12345",
        user="tsurugi",
        password="password",
        default_timeout=30,  # seconds
    ) as connection:
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
