# Tsurugi Python DB-API 使用方法

Tsurugi Python DB-API（tsurugi-dbapi）の基本的な使用方法を説明します。

## 概要

Tsurugi Python DB-API（tsurugi-dbapi）はPythonのDB API 2.0（PEP 249）に準拠し、Tsurugi独自の拡張機能を加えています。

Pythonプログラムからはtsurugi_dbapiモジュールをインポートしてください。

```python
import tsurugi_dbapi as tsurugi
```

### 制限事項

- TCP接続のみ利用可能です。
- BLOB/CLOBには対応していません。

## Tsurugiへの接続方法

Tsurugiへの接続には `connect` 関数を使用します。

接続オプション（接続先を表すエンドポイントやユーザー・パスワード等）を指定する方法は、Configクラスを使う方法と、`connect` 関数の引数に指定する方法があります。  
（後者の方法でも、内部ではConfigオブジェクトを構築します。設定できる内容については TODO/tsurugi_dbapi/index.html#tsurugi_dbapi.Config を参照してください）

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
tsurugi-dbapiの拡張機能です。

```python
        table_names = connection.list_tables()
```

## テーブル定義取得

Connectionの `get_table_metadata` メソッドで、テーブルの定義を取得できます。
tsurugi-dbapiの拡張機能です。

```python
        metadata = connection.get_table_metadata("example")
        print("table_name:", metadata.table_name)
        print("primary_keys:", metadata.primary_keys)

        columns = metadata.columns
        print("columns.len:", len(columns))
        for column in columns:
            print(column.name, column.type_code, column.sql_type)
```

`find_table_metadata` という拡張メソッドも提供しています。  
`get_table_metadata` との違いは、指定されたテーブルが存在しないとき、`get_table_metadata`  ではエラーが発生しますが `find_table_metadata` はNoneを返します。

テーブル定義から取得できる情報については TODO/tsurugi_dbapi/index.html#tsurugi_dbapi.TableMetadata を参照してください。

## トランザクションの使用方法

トランザクションが開始されていない状態でSQLを実行すると、暗黙にトランザクションが開始されます。

Tsurugiではトランザクション開始時にトランザクションオプションを指定しますが、tsurugi-dbapiのデフォルトでは、トランザクション種別OCCのトランザクションオプションが使用されます。

Connectionの `commit` または `rollback` メソッドを実行することにより、トランザクションは終了します。

> [!TIP]
>
> SQLの実行中にエラーが発生すると、そのトランザクションの状態がINACTIVE_TRANSACTIONになり、継続して使用する（SQLを実行する）ことができなくなることがあります。
>
> この場合、`rollback` を実行するとトランザクションがクローズされ、引き続きCursorを使ってSQLを実行する（新しいトランザクションを開始する）ことができます。

> [!NOTE]
>
> オートコミット（SQLを実行する度に自動的にコミットする）の機能はありません。

### トランザクションオプション

トランザクションオプションはTransactionOptionクラスで扱います。  
TransactionOptionオブジェクトの生成方法および設定内容については TODO/tsurugi_dbapi/index.html#tsurugi_dbapi.TransactionOption を参照してください。

トランザクションオプションは、ConfigまたはConnectionに設定しておくことができます。

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

> [!IMPORTANT]
>
> Tsurugiでは、select文のみのトランザクションであっても、必ずコミットする必要があります。
>
> （Tsurugiでは、select文のみのトランザクションであっても（他のトランザクションと競合して）シリアライゼーションエラーが発生することがあるので、コミットが成功することを確認する必要があります）

### 更新系SQL・DDLの例

更新系SQL（insert, update, delete）やDDLを実行するには、Cursorの `execute` メソッドを使用します。

更新系SQLが処理した件数は Cursorの `rowcount` 属性で取得できます。

```python
    with connection.cursor() as cursor:
        cursor.execute("drop table if exists example")
        cursor.execute("create table example (foo int primary key, bar bigint, zzz varchar(10))")
        connection.commit()

        cursor.execute("insert into example values (1, 100, 'abc'), (2, 200, 'def'), (3, 300, 'ghi')")
        print("insert rowcount:", cursor.rowcount)
        connection.commit()
```

> [!IMPORTANT]
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

#### プレースホルダーに対するデータ型の指定

Tsurugiでは、プレースホルダーを定義する際にデータ型を指定する必要があります。

tsurugi-dbapiでは、パラメーターの値を元にデータ型を推定します。  
しかし、Pythonのデータ型からTsurugiのデータ型に一律に変換することはできません。（特にINT, BIGINT, REAL, DOUBLE, DECIMALの区別や、Noneのデータ型の識別ができない）

このため、tsurugi-dbapiでは、Int32, Int64, Float32, Float64, Decimalといったラッパークラスを用意しています。  
これらのラッパークラスをパラメーターに指定すると、データ型を厳密に区別することができます。

どのようなラッパークラスがあるのかについては、TODO/tsurugi_dbapi/type_code/index.html を参照してください。

#### executeで `?` を使用する例

```python
    with connection.cursor() as cursor:
        sql = "select * from example where foo = ?"
        parameters = (tsurug.type_code.Int32(1),)
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
            (tsurugi.type_code.Int32(1), tusurig.Int64(100), tsurugi.type_code.Str("abc")),
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
            {"foo": tsurugi.type_code.Int32(1), "bar": tsurugi.type_code.Int64(100), "zzz": tsurugi.type_code.Str(None)},
            {"foo": 2, "bar": 200, "zzz": "def"},
            {"foo": 9, "bar": 900, "zzz": "xyz"},
        ]
        cursor.executemany(sql, parameters_list)

        connection.commit()
```

#### prepareの例

tsurugi-dbapi独自の拡張機能として、SQL実行前にプレースホルダーのデータ型を指定しておくことができます。

`prepare` メソッドでプレースホルダーのデータ型を指定しておくと、後続の `execute` や `executemany` メソッドのパラメーターでは厳密なデータ型を指定する必要がありません。

```python
    with connection.cursor() as cursor:
        sql = "select * from example where foo = ?"
        cursor.prepare(sql, (tsurug.type_code.Int32,))
        parameters = (1,)
        cursor.execute(sql, parameters)
        row = cursor.fetchone()

        connection.commit()
```

```python
    with connection.cursor() as cursor:
        sql = "insert into example values (:foo, :bar, :zzz)"
        cursor.prepare(sql, {
            "foo": tsurugi.type_code.Int32,
            "bar": tsurugi.type_code.Int64,
            "zzz": tsurugi.type_code.Str
        })
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
> tsurugi-dbapiでは、`execute`, `executemany`, `prepare` メソッドが新しいSQL文で呼ばれる度に、TsurugiのPreparedStatementを生成してCursorオブジェクト内部にキャッシュします。
> 同じSQL文で `execute`, `executemany` メソッドが呼ばれると、キャッシュされていたPreparedStatementを再利用します。
> 
>このため、ひとつのCursorオブジェクトで様々なSQLを実行していると、PreparedStatementが溜まり続けます。
> 
>Cursorの `close` または `clear` メソッドを呼ぶと、キャッシュされていたPreparedStatementは解放されます。

## 例外クラス

tsurugi-dbapiの例外クラスは、DB API 2.0 (PEP 249)で定義されている例外クラスを継承しています。

このため、[Tsubakuro/Java](https://github.com/project-tsurugi/tsubakuro) の例外クラスの継承関係とは異なっている部分があります。

## ログ出力

tsurugi-dbapi（の中核である内部モジュール `tsubakuro_rust_python` ）はログを出力しますが、デフォルトではログは無効になっています。  
`env_logger_init` 関数を呼ぶことで、実際にログが出力されるようになります。

`env_logger_init` 関数の第1引数でフィルター（ログレベル）を指定します。設定内容はenv_loggerの環境変数RUST_LOGに指定する内容と同様です。（具体的な設定値については [env_loggerのドキュメント](https://docs.rs/env_logger/0.11.8/env_logger/) を参照してください）  
第2引数でログ出力先となるファイルのパスを指定します。省略した場合は標準エラーに出力されます。

```python
    tsurugi.env_logger_init("tsubakuro_rust_python=trace")
```

```python
    tsurugi.env_logger_init("tsubakuro_rust_python=trace", "/tmp/tsurugi-dbapi.log")
```

なお、 `env_logger_init` 関数の呼び出しは一度のみ有効です。2回目以降の呼び出しは無視されます。（ログ出力の設定を変更することはできません）
