from tsurugi_dbapi._tsubakuro_rust_python import type_code as _rust

for name in _rust.__all__:
    globals()[name] = getattr(_rust, name)

__all__ = _rust.__all__
