from tsubakuro_rust_python._tsubakuro_rust_python import error as _rust

for name in _rust.__all__:
    globals()[name] = getattr(_rust, name)

__all__ = _rust.__all__
