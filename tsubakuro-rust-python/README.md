# tsubakuro-rust-python (Tsurugi Python library)

tsubakuro-rust-python is a Python library to access [Tsurugi](https://github.com/project-tsurugi/tsurugidb).

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
import tsubakuro_rust_python as tsurugi

def example():
    config = tsurugi.Config()
    config.endpoint = "tcp://localhost:12345"
    config.user = "tsurugi"
    config.password = "password"
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

## How to build

Since it depends on [tsubakuro-rust-core](../tsubakuro-rust-core), its build environment (e.g. `rustc`, `cargo`, `protoc`) is required.

### Generate `.pyi` file

```bash
cd tsubakuro-rust-python
cargo run --bin stub_gen

ls tsubakuro_rust_python.pyi
```

### Generate `.whl` file

Install [`maturin`](https://github.com/PyO3/maturin) beforehand.

```bash
uv tool install maturin
```

> [!NOTE]
>
> To include a stub file in the wheel file, generate `tsubakuro_rust_python.pyi` first.

```bash
cd tsubakuro-rust-python
uv run maturin build --release

ls target/wheels/
```

### Generate API html

Install [pdoc3](https://github.com/pdoc3/pdoc) beforehand.

```bash
cd tsubakuro-rust-python
uv pip install pdoc3
```

```bash
cd tsubakuro-rust-python
uv run pdoc tsubakuro_rust_python --html -o docs/ --force

ls docs/tsubakuro_rust_python/
```

## License

[Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)