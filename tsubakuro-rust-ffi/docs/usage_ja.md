# tsubakuro-rust-ffiの使用方法

当文書では、tsubakuro-rust-ffiの基本的な使用方法を説明します。

## はじめに

tsubakuro-rust-ffiは、[tsubakuro-rust-core](../../tsubakuro-rust-core) をC言語（Rust以外の言語）から呼び出すためのC ABI形式の関数群を提供するライブラリー（クレート）です。

以降、当文書では、tsubakuro-rust-ffiが提供する構造体・関数のことを「FFI構造体」「FFI関数」と呼びます。

tsubakuro-rust-ffiは言わばtsbakuro-rust-coreのラッパーであり、基本的にはFFI構造体/FFI関数はtsubakuro-rust-coreの構造体/メソッドと1対1に対応しています。（RustとC言語の違いもあり、一致していないところもあります） 
したがって、Tsurugi DBにアクセスするための手順（メソッドの呼び出し順や機能）等については、tsubakuro-rust-coreのドキュメントを参照してください。

tsubakuro-rust-ffiをビルドするとsoファイル（Linuxの場合）やdllファイル（MS-Windowsの場合）が生成されます。 
また、cbindgenというツールを使って、FFI関数が宣言された「C言語のヘッダーファイル」を生成することもできます。 
これらの生成ファイルそのものはtsubakuro-rust-ffiでは提供しませんので、ユーザーが自身の環境に合わせて生成してください。



## tsubakuro-rust-ffiの基本ルール

### FFI構造体

#### FFI構造体の名前

tubakuro-rust-ffiでは、構造体はハンドルという形（ポインター）で扱います。

ハンドルの名前は、tsubakuro-rust-coreの構造体名に対し、接頭辞として `TsurugiFfi` 、接尾辞として `Handle` を付加したものになります。

例えば `ConnectionOption` は `TsurugiFfiConnectionOptionHandle` です。

#### FFI構造体の内容

tubakuro-rust-ffiでは、構造体の内容（フィールド）を公開しません。 
構造体を操作するFFI関数を提供します。

#### FFI構造体のライフサイクル

FFI構造体のメモリーの確保（インスタンスの生成）および解放はtsubakuro-rust-ffi側が行います。

FFI構造体のハンドルを受け取ったら、**使用終了後に必ず一度だけdispose関数を呼ぶ必要があります** 。
これにより、FFI構造体のメモリーを解放（インスタンスを破棄）します。

dispose関数を呼ばなかった場合はリソースがリークします。2回以上呼んだ場合の動作は不定です。（たぶんアプリケーションがクラッシュします）

### FFI関数

#### FFI関数の名前

FFI関数の名前は、tsubakuro-rust-coreの構造体名およびメソッド名に対し、以下の要素をアンダースコア `_` で繋いだものになります。 

- 接頭辞として `tsurugi_ffi` 
- 次に構造体名をsnake_caseにしたもの
- 最後にメソッド名
  - 値を取得するメソッドで名詞のみの場合は、メソッド名の前に `get_` を付加する
    - 例えば `SqlColumn` の `name` は `tsurugi_ffi_sql_column_get_name`
  - 左辺値によって返す型が異なるメソッドでは、型が分かるような名前に変更する。基本的にはメソッド名の末尾に型名を付与する
    - 例えば `SqlClient` を返す `Session` の `make_client` は `tsurugi_ffi_session_make_sql_client`
    -  `SqlQueryResult` の `fetch` は `tsurugi_ffi_sql_query_result_fetch_int4` や `_fetch_character`

例えば `Session` の `connect` は `tsurugi_ffi_session_connect` です。 

FFI構造体のインスタンスを生成するFFI関数では、メソッド名に当たる部分は `create` になります。
また、FFI構造体を解放するFFI関数は、メソッド名に当たる部分は `dispose` です。

例えば `ConnectionOption` のインスタンスを生成してハンドルを返すFFI関数は `tsurugi_ffi_connection_option_create` で、解放するFFI関数は `tsurugi_ffi_connection_option_dispose` です。

#### FFI関数の返り値

ほぼ全てのFFI関数は、返り値の型は32ビット符号なし整数（C言語ではuint32_t）で、処理が正常終了した場合は0が返ります。
エラー時の値については後述のリターンコードを参照してください。

dispose関数だけは返り値がありません（voidです）。

#### FFI関数の引数

FFI関数の引数は、基本的に以下のような順序になります。

1. コンテキスト構造体
   - TsurugiFfiContextHandle（後述）
2. レシーバーオブジェクト
   - 処理対象となるFFI構造体のハンドル
3. tsubakuro-rust-coreのメソッドの引数
4. FFI関数の出力を受け取るためのポインター

#### close関数について

tsubakuro-rust-coreでは、closeメソッドが提供されている場合は、Rustのデストラクター（dropメソッド）でリソース解放を行う前にcloseメソッドを呼ぶことを推奨しています。

しかしtsubakuro-rust-ffiではdispose関数の中でcloseメソッドを呼ぶようになっているので、ユーザーが明示的にclose関数を呼ぶ必要はありません。（エラーのハンドリングを自分で行いたい場合はclose関数を呼んでください）

### 文字列の扱い

tsubakuro-rust-ffiでは、文字列はNUL終端（ `\0` で終わる）のUTF-8で扱います。

文字列をFFI関数に渡す場合は、文字列の領域は呼び出し側で確保して、その先頭アドレスをFFI関数に渡してください。
FFI関数内部で文字列データをコピーするので、FFI関数の呼び出しから返ったら領域を解放して構いません。

FFI関数から文字列を返す場合は、文字列の領域は取得元FFI構造体の中に確保されます。
したがって、返されたポインターが指す領域を呼び出し元が解放する必要はありません。
取得元FFI構造体がdispose関数によって解放されるときに一緒に解放されます。

なお、文字列を返す同じ関数を2回以上呼んだ場合は、2回目の呼び出し時に1回目の文字列の領域が解放されます。
したがって、返されたポインターを保持し続けるのは危険です。なるべく、呼び出し側の領域に文字列データをコピーしてください。

同じ理由により、文字列を返す関数はスレッドアンセーフです。

### バイト配列の扱い

tsubakuro-rust-ffiでは、バイト配列は先頭アドレスとバイト数のペアで扱います。
バイト数が0の場合はアドレスは無視します（nullで構いません）。

FFI関数内部でのバイト配列の扱い方は文字列と同様ですので、注意点も文字列と同じです。

### リストの扱い

FFI関数の中には、文字列一覧や数値一覧、カラム一覧など、リストを扱うものがあります。

リストは基本的に要素の配列の先頭アドレス（文字列の場合は二重ポインターになる）と要素数のペアで扱います。
要素数が0の場合はアドレスは無視します（nullで構いません）。 
これらの一覧を返すFFI関数では、取得元FFI構造体の領域に配列を保持します。すなわち、注意点は文字列と同じです。

ただし、カラム一覧などの「FFI構造体のハンドルの一覧」となるものは別です。 
ハンドルは使い終わったらdispose関数を呼んで解放する必要がありますが、取得元FFI構造体の中で保持してしまうとdispose関数を呼ぶ必要が無くなり、同じハンドルなのにdispose関数を呼ぶべきかどうか混乱してしまいます。 

そこで、ハンドルの一覧の場合はFFI構造体の中では保持しません。
要素数を返すFFI関数と、要素のインデックスを受け取って要素のハンドルを返すFFI関数を提供します。
呼び出し元は、返されたハンドル1個1個に対してdispose関数を呼んでください。



## tsubakuro-rust-ffiの個別仕様

### リターンコード（TsurugiFfiRc）

ほぼ全てのFFI関数はリターンコードを返すようになっていて、処理結果のステータスを表します。
リターンコードは符号なし32ビット整数で、以下のようなルールです。

- 0 - 正常
- 0以外 - エラー
  - 上位2ビットでエラーの発生個所を表す
    - 00 - 正常
    - 01 - FFIエラー（主に、FFI関数の引数の値が異常）
    - 10 - tsubakuro-rust-coreのクライアントエラー
    - 11 - tsubakuro-rust-coreのサーバーエラー
  - FFIエラーの場合、上位2ビットの次の6ビットが0なら、FFI関数呼び出しの引数エラー。下位ビットがエラーのあった引数のインデックスを表す
  - サーバーエラーの場合、下位20ビットがエラーコード、その上がエラーカテゴリーを表す。例えばSQL-02001のエラーの場合、エラーカテゴリーは3（SQL）, エラーコードは2001

### ログ出力（tsurugi_ffi_env_logger_init関数）

Tsubakuro/Rustは、logクレートを使ってログ出力を行うようになっています。
しかしデフォルトではログ出力の実装が無いため、何も出力されません。

tsubakuro-rust-ffiでは、ログ出力の実装としてenv_loggerクレートを使えるようになっています。

tsurugi_ffi_env_logger_init関数を呼ぶとenv_loggerが有効になります。
env_loggerは環境変数RUST_LOGの設定に応じてコンソールにログを出力します。

RUST_LOGでトレースレベルのログを出力するよう設定してtsurugi_ffi_env_logger_init関数を呼び出せば、アプリケーションを実行した際に以下のようなログが出るので、FFI関数を呼び出せているかどうかを確認することができます。

```bash
$ export RUST_LOG=tsubakuro_rust_ffi=trace
$ # execute application using tsubakuro-rust-ffi
～
[2025-02-12T09:01:56.966Z TRACE tsubakuro_rust_ffi::logger] tsurugi_ffi_env_logger_init() end
```

### コンテキスト構造体（TsurugiFfiContextHandle）

ほとんどのFFI関数では、第1引数にコンテキスト構造体（TsurugiFfiContextHandle）を渡すようになっています。
（数少ない例外は、コンテキスト構造体自身を対象とするFFI関数と、dispose関数です）

コンテキスト構造体を指定して呼び出したFFI関数でエラーが発生した場合、コンテキスト構造体の中にエラー情報が格納されます。
エラーコードやエラーメッセージ等はコンテキスト構造体から取得することができます。

その性質から、コンテキスト構造体のインスタンスはスレッド毎に用意するのが良いでしょう。
FFI関数の呼び出し毎に新しいインスタンスを作っても構いませんが。
少なくとも、複数スレッド間で共有すべきではありません。

コンテキスト構造体を渡す第1引数にnullを指定してFFI関数を呼び出すことも可能ですが、その場合は、エラーが発生してもエラー情報を取得することはできません。

コンテキスト構造体自身を対象とするFFI関数の場合、その中でエラーが発生しても、そのコンテキスト構造体の内部情報が書き変わることはありません。

### ジョブ構造体（TsurugiFfiJobHandle）

tsubakuro-rust-coreの `Job<T>` に当たるFFI構造体のハンドルはTsurugiFfiJobHandleです。

`Job<T>` には何の構造体を扱っているのかを表すジェネリクスTが付いていますが、C言語にはジェネリクスが無いため、TsurugiFfiJobHandle単独では何の構造体を扱っているのか分かる術はありません。

どのFFI関数を呼び出して取得したTsurugiFfiJobHandleなのかに応じて、take関数等では適切なハンドルにキャストしてください。