# Tsurugi Pythonライブラリー使用方法

[Tsurugi](https://github.com/project-tsurugi/tsurugidb) のPythonクライアントライブラリー（tsubakuro-rust-python）の基本的な使用方法を説明します。

## 概要

tsubakuro-rust-pythonはPythonのDB API 2.0（PEP 249）に準拠し、Tsurugi独自の拡張機能を加えています。

Pythonプログラムからはtsubakuro_rust_pythonモジュールをインポートしてください。

```python
import tsubakuro_rust_python as tsurugi
```

### 制限事項

- TCP接続のみ利用可能です。
- BLOB/CLOBには対応していません。

## Tsurugiへの接続方法

Tsurugiへの接続には `connect` 関数を使用します。

接続オプション（接続先を表すエンドポイントやユーザー・パスワード等）を指定する方法は、Configクラスを使う方法と、`connect` 関数の引数に指定する方法があります。  
（後者の方法でも、内部ではConfigオブジェクトを構築します）

`connect` 関数はConnectionオブジェクトを返します。

Connectionオブジェクトの使用が終わったら `close` メソッドを呼ぶ必要があります。  
with文を使用すると、自動的に `close` メソッドが呼ばれます。

#### Configを使用する例

```python
    config = tsurugi.Config()
    config.endpoint = "tcp://localhost:12345"
    config.user = "tsurugi"
    config.password = "password"
    with tsurugi.connect(config) as connection:
        pass
```

```python
    config = tsurugi.Config(endpoint="tcp://localhost:12345", user="tsurugi", password="password")
    with tsurugi.connect(config) as connection:
        pass
```

#### connect関数の引数で指定する例

```python
    with tsurugi.connect(endpoint="tcp://localhost:12345", user="tsurugi", password="password") as connection:
        pass
```

## テーブル一覧取得

Connectionの `list_tables` メソッドで、テーブル名の一覧を取得できます。
tsubakuro-rust-pythonの独自機能です。

```python
        table_names = connection.list_tables()
```

## テーブル定義取得

Connectionの `get_table_metadata` メソッドで、テーブルの定義を取得できます。
tsubakuro-rust-pythonの独自機能です。

```python
        metadata = connection.get_table_metadata("example")
        print("table_name:", metadata.table_name)
        print("primary_keys:", metadata.primary_keys)

        columns = metadata.columns
        print("columns.len:", len(columns))
        for column in columns:
            print(column.name, column.type_code, column.sql_type)
```

`find_table_metadata` というメソッドもあります。  
`get_table_metadata` との違いは、指定されたテーブルが存在しないとき、`get_table_metadata`  ではエラーが発生しますが `find_table_metadata` はNoneを返します。

## トランザクションの使用方法

トランザクションが開始されていない状態でSQLを実行すると、暗黙にトランザクションが開始されます。

Tsurugiにはトランザクションの種類がいくつかありますが、デフォルトのトランザクション種別はOCCです。

Connectionの `commit` または `rollback` メソッドを実行することにより、トランザクションは終了します。

> [!NOTE]
>
> オートコミット（SQLを実行する度に自動的にコミットする）の機能はありません。

### トランザクションオプション

トランザクション開始時に使用されるトランザクションオプションは、ConfigまたはConnectionに設定しておくことができます。

#### トランザクションオプションをConfigで設定しておく例

```python
    tx_option = tsurugi.TransactionOption.ltx(write_preserve=["example"])
    config.transaction_option = tx_option
```

#### トランザクションオプションをConnectionに設定する例

```python
    tx_option = tsurugi.TransactionOption.ltx(write_preserve=["example"])
    connection.transaction_option = tx_option
```

トランザクションオプションをConnectionに設定する場合、現在実行中のトランザクションには影響しません。  
次のトランザクションが開始されるときから適用されます。

## SQLの実行方法

SQLを実行するにはCursorオブジェクトを使用します。

CursorオブジェクトはConnectionから生成します。

Cursorオブジェクトの使用が終わったら `close` メソッドを呼ぶ必要があります。  
with文を使用すると、自動的に `close` メソッドが呼ばれます。

### selectの例

select文を実行するには、Cursorの `execute` メソッドを使用します。

select文の実行結果は `fetch` 系メソッドで取得します。

```python
    with connection.cursor() as cursor:
        cursor.execute("select * from example")
        while True:
            row = cursor.fetchone()
            if row is None:
                break
            print("row:", row)

        connection.commit()
```

```python
    with connection.cursor() as cursor:
        cursor.execute("select * from example")
        rows = cursor.fetchall()
        print("rows:", rows)

        connection.commit()
```

```python
    with connection.cursor() as cursor:
        cursor.execute("select * from example")
        for row in cursor:
            print("row:", row)

        connection.commit()
```

select結果の1行はタプルで返ります。

### 更新系SQL・DDLの例

更新系SQL（insert, update, delete）やDDLを実行するには、Cursorの `execute` メソッドを使用します。

更新系SQLが処理した件数は Cursorの `rowcount` 属性で取得できます。

```python
    with connection.cursor() as cursor:
        cursor.execute("drop table if exists example")
        cursor.execute("create table example (foo int primary key, bar bigint, zzz varchar(10))")
        connection.commit()

        cursor.execute("insert into tsubakuro_rust_python_example values (1, 100, 'abc'), (2, 200, 'def'), (3, 300, 'ghi')")
        print("insert rowcount:", cursor.rowcount)
        connection.commit()
```

> [!NOTE]
>
> Tsurugiでは、DDLを実行する際もトランザクションが開始されるので、コミットする必要があります。
>
> なお、現在のTsurugiにはDDLとDMLを同一のトランザクションで混在できないという制限があるので、create tableとinsertは異なるトランザクションで実行する必要があります。

### パラメーターを使用する例

SQL文の中にプレースホルダーを記述し、実行する際にパラメーターを指定することができます。

`execute` メソッドでは、1回分のパラメーターを指定します。  
`executemany` メソッドは、複数回分のパラメーターを指定できます。

プレースホルダーの指定方法には、`?` を指定する方式と名前（ `:name` ）を指定する方式があります。  
`?` を使用する場合、パラメーターは値の一覧（タプル）になります。  
名前を指定する場合、パラメーターは「keyが名前, valueが値」の辞書（dict）になります。

> [!NOTE]
>
> Tsurugiでは、プレースホルダーを定義する際にデータ型を指定する必要があります。
>
> tsubakuro-rust-pythonでは、パラメーターの値を元にデータ型を推定しています。
> しかし、Pythonのデータ型からTsurugiのデータ型に一律に変換することはできません。（特にINT, BIGINT, REAL, DOUBLE, DECIMALの区別や、Noneのデータ型の識別ができない）
>
> このため、tsubakuro-rust-pythonでは、Int32, Int64, Float32, Float64, Decimalといったラッパークラスを用意しています。
> これらのラッパークラスをパラメーターに指定すると、データ型を厳密に区別することができます。

#### executeで `?` を使用する例

```python
    with connection.cursor() as cursor:
        sql = "select * from example where foo = ?"
        parameters = (tsurug.Int32(1),)
        cursor.execute(sql, parameters)
        row = cursor.fetchone()
        print(row)

        connection.commit()
```

#### executemanyで `?` を使用する例

```python
    with connection.cursor() as cursor:
        sql = "insert into example values (?, ?, ?)"
        parameters_list = [
            (tsurugi.Int32(1), tusurig.Int64(100), tsurugi.Str("abc")),
            (2, 200, "def"),
            (9, 900, "xyz"),
        ]
        cursor.executemany(sql, parameters_list)

        connection.commit()
```

> [!TIP]
>
> Tsurugi用のプレースホルダーのデータ型を確定するために参照するのは、先頭のパラメーターだけです。
>
> したがって、厳密なデータ型を指定するためにInt32やInt64といったラッパークラスを使うのは、先頭のパラメーターのみで構いません。

#### `:name` を使用する例

```python
    with connection.cursor() as cursor:
        sql = "insert into example values (:foo, :bar, :zzz)"
        parameters_list = [
            {"foo": tsurugi.Int32(1), "bar": tsurugi.Int64(100), "zzz": tsurugi.Str(None)},
            {"foo": tsurugi.Int32(2), "bar": tsurugi.Int64(200), "zzz": tsurugi.Str("def")},
            {"foo": tsurugi.Int32(9), "bar": tsurugi.Int64(900), "zzz": tsurugi.Str("xyz")},
        ]
        cursor.executemany(sql, parameters_list)

        connection.commit()
```

> [!NOTE]
>
> Tsurugi用のプレースホルダーのデータ型を確定するために参照するのは、先頭のパラメーターだけです。
>
> tsubakuro-rust-pythonが対象としているバージョンのPythonでは辞書（dict）の並び順が保証されるようですが、この例では、念のために全ての値にラッパークラスを使用しています。

#### prepareの例

tsubakuro-rust-pythonの独自機能として、SQL実行前にプレースホルダーのデータ型を指定しておくことができます。

`prepare` メソッドでプレースホルダーのデータ型を指定しておくと、後続の `execute` や `executemany` メソッドのパラメーターでは厳密なデータ型を指定する必要がありません。

```python
    with connection.cursor() as cursor:
        sql = "insert into example values (:foo, :bar, :zzz)"
        cursor.prepare(sql, {"foo": tsurugi.Int32, "bar": tsurugi.Int64, "zzz": tsurugi.Str})
        parameters_list = [
            {"foo": 1, "bar": 100, "zzz": None},
            {"foo": 2, "bar": 200, "zzz": "def"},
            {"foo": 9, "bar": 900, "zzz": "xyz"},
        ]
        cursor.executemany(sql, parameters_list)

        connection.commit()
```

> [!IMPORTANT]
>
> tsubakuro-rust-pythonでは、`execute`, `executemany`, `prepare` メソッドが新しいSQL文で呼ばれる度に、TsurugiのPreparedStatementを生成してCursorオブジェクト内部にキャッシュします。
> 同じSQL文で `execute`, `executemany` メソッドが呼ばれると、キャッシュされていたPreparedStatementを再利用します。
> 
>このため、ひとつのCursorオブジェクトで様々なSQLを実行していると、PreparedStatementが溜まり続けます。
> 
>Cursorの `close` または `clear` メソッドを呼ぶと、キャッシュされていたPreparedStatementは解放されます。

## ログ出力

tsubakuro-rust-pythonは [env_logger](https://crates.io/crates/env_logger) を使ってログ出力していますが、デフォルトではどこにも出力されません。  
`env_logger_init` 関数を呼ぶことで、実際にログが出力されるようになります。

`env_logger_init` 関数の第1引数でフィルター（ログレベル）を指定します。設定内容はenv_loggerの環境変数RUST_LOGに指定する内容と同様です。  
第2引数でログ出力先となるファイルのパスを指定します。省略した場合は標準エラーに出力されます。

```python
    tsurugi.env_logger_init("tsubakuro_rust_python=trace")
```

```python
    tsurugi.env_logger_init("tsubakuro_rust_python=trace", "/tmp/tsubakuro-rust-python.log")
```

なお、 `env_logger_init` 関数の呼び出しは一度のみ有効です。2回目以降の呼び出しは無視されます。（ログ出力の設定を変更することはできません）

