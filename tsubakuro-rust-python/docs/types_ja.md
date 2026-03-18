## Tsurugi Python DB-API データ型

Tsurugi Python DB-API（tsurugi-dbpai）で使用できるデータ型について説明します。

## データ型の対応表

| Tsurugiの型              | Pythonの型                      | type_code      | 備考   |
| ------------------------ | ------------------------------- | -------------- | ------ |
| BOOLEAN                  | bool                            | Bool           |        |
| INT                      | int                             | Int32          |        |
| BIGINT                   | int                             | Int64          |        |
| REAL                     | float                           | Float32        |        |
| DOUBLE                   | float                           | Float64        |        |
| DECIMAL                  | decimal.Decimal                 | Decimal        |        |
| CHAR, VARCHAR            | str                             | Str            |        |
| BINARY, VARBINARY        | bytes                           | Bytes          |        |
| BLOB                     |                                 |                | 未対応 |
| CLOB                     |                                 |                | 未対応 |
| DATE                     | datetime.date                   | Date           |        |
| TIME                     | datetime.time（tzinfoなし）     | Time           |        |
| TIMESTAMP                | datetime.datetime（tzinfoなし） | Datetime       |        |
| TIME WITH TIME ZONE      | datetime.time（tzinfoあり）     | OffsetTime     |        |
| TIMESTAMP WITH TIME ZONE | datetime.datetime（tzinfoあり） | OffsetDatetime |        |

- selectの実行結果は、この表のPythonの型で返ります。
- `execute`, `executemany` メソッドのパラメーターとしてPythonの型の値が使用されたとき、Tsurugiのプレースホルダーの型としては、intはBIGINT, floatはDOUBLEとして扱われます。
- BLOB, CLOBは、現在のtsubakuro-rust-pythonでは未対応です。

## type_code

type_codeは、テーブル定義（Connectionの `get_table_metadata` メソッドで取得できる）のカラム情報やselectの実行結果のカラム情報（Cursorの `description` 属性で取得できる）で取得できる値です。

また、Tsurugi Python DB-APIではtype_codeと同名のラッパークラスを用意しています。  
ラッパークラスは、パラメーター付きの `execute` や`executemany`  メソッドを呼ぶ際に、パラメーターの厳密な型を指定するために利用できます。

（ラッパークラスの利用例は [usage_ja.md](usage_ja.md) を参照してください）

## TIME, TIMESTAMPの小数秒精度

TsurugiのTIMEやTIMESTAMPはナノ秒（9桁）まで保持できますが、Pythonのtimeやdatetimeはマイクロ秒（6桁）までしか保持できません。

selectの実行結果として返るtimeやdatetimeでは、マイクロ秒より下の桁は切り捨てられます。

`execute`, `executemany` メソッドのパラメーターでtimeやdatetimeを使用する場合も、マイクロ秒までしか指定できません。  
ラッパークラスを使うと、ナノ秒まで指定することができます。

```python
import tsurugi_dbapi as tsurugi
import datetime

     # 2026-02-06 12:34:56.123456789
     value = tsurugi.type_code.Datetime(datetime.datetime(2026, 2, 6, 12, 34, 56), 123456789)
     value = tsurugi.type_code.Datetime.of(2026, 2, 6, 12, 34, 56, 123456789)
```

