# 型スタブファイル

Pythonの型スタブファイルは、型ヒントを記述したファイル。

配布用のwheelファイル内にも入れるので、wheelファイル生成前に生成する。

## 型スタブファイルの生成方法

tsubakuro-rust-pythonでは [pyo3-stub-gen](https://crates.io/crates/pyo3-stub-gen) を使って生成する。

が、pyo3-stub-gen 0.18.0 には [バグ（#384）](https://github.com/Jij-Inc/pyo3-stub-gen/issues/384) があり、例外クラスが正しく出力されない。
（ `builtins.AnalyzeException` のように、ビルトインでないのに `builtins.` が付いてしまう）

このため、pyo3-stub-genによって型スタブファイルを生成した後に、修正用のスクリプトを実行する。

```bash
cd tsubakuro-rust-python
cargo run --bin stub_gen
uv run tools/modify_pyi.py -d python
```

