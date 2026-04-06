# Tsurugi Python DB-API

Tsurugi Python DB-API (tsurugi-dbapi) is a Python DB API 2.0 driver for accessing [Tsurugi](https://github.com/project-tsurugi/tsurugidb).

## Target

- Tsurugi 1.10.0 or later
- Python 3.10 or later
- Python DB API 2.0 (PEP 249)

## Limitation

- Only TCP connection is available.
- The default transaction_type is `OCC`.
- The default commit_type is `DEFAULT`.
- The default shutdown_type is `GRACEFUL`.
- BLOB/CLOB is not supported.

## Installation

```bash
uv add tsurugi-dbapi
```

## Example

```python
import tsurugi_dbapi as tsurugi

def example():
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

See [examples](https://github.com/project-tsurugi/tsubakuro-rust/tree/master/tsubakuro-rust-python/examples) for more examples.

## License

[Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)