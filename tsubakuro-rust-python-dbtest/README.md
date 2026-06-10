# tsubakuro-rust-python-dbtest

tsubakuro-rust-python-dbtest is a project for testing [tsubakuro-rust-python](../tsubakuro-rust-python).

## How to test

```bash
cd tsubakuro-rust-python-dbtest
uv run pytest --endpoint=tcp://localhost:12345
```

#### Example of blob relay service endpoint

```bash
uv run pytest --endpoint=tcp://localhost:12345 --blob-relay-service-endpoint=http://localhost:52345
```

> [!IMPORTANT]
>
> tsubakuro-rust-python ([tsubakuro-rust-core](../tsubakuro-rust-core)) does not support `dns:///`.
> Use `http://` instead.

#### Example of blob relay service CA certificate PEM file

```bash
uv run pytest --endpoint=tcp://localhost:12345 --blob-relay-service-endpoint=https://localhost:52345 --blob-relay-service-ca-cert-pem-file=/path/to/pem
```

