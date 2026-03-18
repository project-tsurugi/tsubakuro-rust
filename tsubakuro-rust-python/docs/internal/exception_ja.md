# Tsurugi Python DB-API 例外クラス

[tsubakuro-rust-python/src/error.rs](../../src/error.rs) で例外クラスを実装している。

例外クラスはPython DB API 2.0（PEP 249）で定義されているものと、Tsurugiのエラーがある。

Tsurugiのエラーは [Tsubakuro/Java](https://github.com/project-tsurugi/tsubakuro) のSQLクライアントの例外クラスを元にしている。  
これらはTsurugiで規定している継承関係を持っているが、tsubakuro-rust-pythonではDB API 2.0で規定されている継承関係にしている。

## ソースコードの生成方法

Tsubakuro/JavaのSQLクライアントの例外クラスは数が多いので、ツールでRustのコードを生成する。

```bash
cd tsubakuro-rust-dev-java
./gradlew runTsubakuroRustPythonExceptionGenerator
```

標準出力にスニペットが出力されるので、error.rsに転記する。

> [!NOTE]
>
> Tsubakuro/JavaのSQLクライアントの例外クラスに変更があったら、この作業を行う想定。