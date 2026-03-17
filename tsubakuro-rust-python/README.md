# Tsurugi Python DB-API

Tsurugi Python DB-API (tsurugi-dbapi) is a Python DB API 2.0 driver for accessing [Tsurugi](https://github.com/project-tsurugi/tsurugidb).

## Target

- Tsurugi 1.9.0 or later
- Python 3.10 or later
- Python DB API 2.0 (PEP 249)

## Limitation

- Only TCP connection is available.
- The default transaction_type is `OCC`.
- The default commit_type is `DEFAULT`.
- The default shutdown_type is `GRACEFUL`.
- BLOB/CLOB is not supported.

## Installation

### Install from Local Source

Since it depends on [tsubakuro-rust-core](../tsubakuro-rust-core), its build environment (e.g. `rustc`, `cargo`, `protoc`) is required.

```bash
mkdir example
cd example
uv init
uv add /path/to/tsubakuro-rust-python

uv run python
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

See [examples](examples) for more examples.

## How to build

Since it depends on [tsubakuro-rust-core](../tsubakuro-rust-core), its build environment (e.g. `rustc`, `cargo`, `protoc`) is required.

### Generate type stub file

After generating the type stub file using `stub_gen`, make changes using `modify_pyi.py`.

```bash
cd tsubakuro-rust-python
cargo run --bin stub_gen
uv run tools/modify_pyi.py -d python

find python -name "*.pyi"
```

### Generate wheel file

Install [`maturin`](https://github.com/PyO3/maturin) beforehand.

```bash
uv tool install maturin
```

> [!NOTE]
>
> To include type stub files in the wheel file, generate stub files first.

```bash
cd tsubakuro-rust-python
uv run maturin build --release

ls target/wheels/
```

### Generate API html

```bash
cd tsubakuro-rust-python
uv run pdoc tsurugi_dbapi --html -o docs/ --force

ls docs/tsurugi_dbapi/
```

## How to test

```bash
cd tsubakuro-rust-python
uv run pytest
```

## License

[Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)