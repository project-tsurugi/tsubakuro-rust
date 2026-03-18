# Tsurugi Python DB-API モジュール名

Tsurugi Python DB-API（tsurugi-dbapi）のモジュール名について。

## モジュール名の変遷

Tsurugi Python DB-API（tsurugi-dbapi）はTsubakuro/Rustを用いたPythonライブラリーであるため、Tsubakuro/Rust内の他のライブラリーと同様の命名方法に基づき、tsubakuro-rust-pythonというライブラリー名で開発している。

このため、開発当初は、tsubakuro-rust-pythonの中核となるRustモジュールの名前は `tsubakuro_rust_python` であり、Pythonとしてのパッケージ名も `tsubakuro-rust-python` （モジュール名は `tsubakuro_rust_python`）であった。

しかしpdoc3によってPythonのAPIドキュメントを生成した際に、RustモジュールがPythonのサブモジュールとして出力された。  
RustモジュールはPythonアプリケーションから見れば内部モジュール（不可視）であるため、APIドキュメントとして公開するのは適切ではない。  
そこで、Rustモジュールの名前を `_tsubakuro_rust_python` に変更した。
（Pythonの慣例として、アンダースコア `_` で始まる名称のものは非公開オブジェクトとして扱われるため、pdoc3の出力対象外になる）

その後、Tsurugi Python DB-APIの公開に先立って、Pythonとしてのパッケージ名を `tsurugi-dbapi` 、モジュール名を `tsurugi_dbapi` とすることになり、そのように変更した。
（ただしRustモジュールの名前は変更していない）

