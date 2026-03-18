# Tsurugi Python DB-API ロギング方針

Tsurugi Python DB-APIのログ出力の方針は以下の通りとする。

## ログ出力の実装

- ログ出力にはlogクレートを使用する。（tsubakuro-rust-coreと同様）
- ログ出力の実装としてenv_loggerを含める。
- env_loggerを初期化するために、Pythonアプリケーションから呼ぶ初期化関数を提供する。
- Pythonアプリケーションから呼ばれる関数やメソッドでは、開始・終了時にTRACEログを出力する。

## env_loggerの初期化方法

env_loggerでは、ログ出力対象のモジュール名やログレベルを指定することができる。

このモジュール名はRustモジュールの名称なので、Tsurugi Python DB-APIでは `_tsubakuro_rust_python` を指定することになる。

ただし、Pythonアプリケーションから呼ぶ初期化関数では `tsubakuro_rust_python` でも指定できるようにする。
（Pythonアプリケーションから呼ぶ初期化関数で `tsubakuro_rust_python` が指定されたら、`_tsubakuro_rust_python` に置換してenv_loggerを初期化する）

参考： [モジュール名の変遷](module-name_ja.md)

## 不採用にした方式

### pyo3-log

Tsurugi Python DB-APIはpyo3クレートを使っており、PyO3は [ログ連携機能](https://pyo3.rs/v0.27.2/ecosystem/logging) を提供している。

そのひとつであるpyo3-logクレートで、Pythonのloggingパッケージと連携することができる。  
これは、Rustのlogクレートから出力したログをPythonのloggingに渡すものである。これにより、loggingのAPIでログレベルを指定できる。

しかし、loggingパッケージではログレベルの最低がDEBUGであり、Tsurugi Python DB-APIの中核であるRustモジュール（ `_tsubakuro_rust_python` ）のログ出力のほとんどがTRACEであることから、pyo3-logクレートでは実質的にほぼ何も出力されない。

このため、pyo3-logクレートは採用しない。

> [!NOTE]
>
> env_loggerと同様にpyo3-log初期化用の関数を提供し、Pythonアプリケーションからどちらを呼ぶかを選択できるようにすることは可能である。
>
> ただし、logクレートでは設定されたロガーを変更することはできない。
>
> このため、「デフォルトをpyo3-logにしておき、実行中に後からenv_loggerに切り替える」といったことはできない。

### pyo3-pylogger

pyo3-pyloggerクレートというものもあるが、これはPythonのloggingから出力したログをRustのlogクレートで処理する（env_loggerに渡す）ものである。

したがって、Rustモジュールから出力するログをPythonアプリケーション側で制御するという目的には適さない。

