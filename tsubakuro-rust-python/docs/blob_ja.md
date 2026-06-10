# Tsurugi Python DB-API BLOB, CLOB使用方法

Tsurugi Python DB-API（tsurugi-dbapi）0.10.0以降で、BLOB, CLOBを使用することができます。

当文書では、BLOB, CLOBをTsurugi Python DB-APIで使用する方法を説明します。

## はじめに

Tsurugiでは、（メモリーに載せるのが躊躇われるような）大きなサイズのデータをBLOB, CLOB型で扱います。
BLOB（binary large object）はバイト列、CLOB（character large object）は文字列が対象です。

以降、BLOB, CLOBを共通で扱う場合は large object（LOB）という用語を用います。

TsurugiはインメモリーDBなので、基本的にデータは全てDBサーバーのメモリー上に置くのですが、LOBデータはメモリー上には置かず、データごとに個別のファイルとして保存されます。

クライアントとTsurugi DBの間でLOBデータを受け渡す方法はいくつかありますが、Tsurugi Python DB-APIではBLOB中継サービスを利用する方法のみをサポートしています。

### BLOB中継サービス

BLOB中継サービスは、クライアントアプリケーションとTsurugi DBの間でのLOBデータの受け渡しをgRPC（TCP/IP）で行います。

なお、この方式が使えるのは、BLOB中継サービスが稼働している場合のみです。

> [!NOTE]
>
> BLOB中継サービスはCLOBも扱います。（CLOB中継サービスというものはありません）

## LOB転送モードの確認方法

LOB転送が行えるか（BLOB中継サービスが利用できるかどうか）は、Connection生成後に以下の方法で確認することができます。

```python
import tsurugi_dbapi as tsurugi

if connection.lob_transfer_type() == tsurugi.LobTransferType.NOT_USE:
    print("The server does not support LOB transfer.")
```

## Tsurugi Python DB-APIにおけるBLOB, CLOBの型

Tsurugi Python DB-APIでは、BLOB, CLOBを以下の表にある型で扱います。

|                                                  | BLOB           | CLOB           |
| ------------------------------------------------ | -------------- | -------------- |
| プリペアードステートメントのパラメーターで使う型 | type_code.Blob | type_code.Clob |
| データの型                                       | bytes          | str            |

以降、当文書ではBLOBの説明のみを記載します。  
Tsurugi Python DB-APIにおけるCLOBの操作方法はBLOBと同等ですので、CLOBについては上記の点を読み替えてください。

## タイムアウト時間の指定

LOBを扱う場合、Tsurugi Python DB-API内部でLOBデータのアップロード・ダウンロードを行います。
そのタイムアウト時間のデフォルト値はConfigで指定できます。

```python
import tsurugi_dbapi as tsurugi

    config = tsurugi.Config()
    config.endpoint = "tcp://localhost:12345"

    config.lob_upload_timeout = 60  # seconds
    config.lob_download_timeout = 60  # seconds
```

## SQL実行メソッドのパラメーターでBLOBを使用する方法

SQL実行メソッドのパラメーターでBLOBを扱う手順は以下のようになります。

1. cursorのアップロードメソッドを使ってBLOBデータをアップロードし、type_code.Blobを取得する。
2. SQL実行メソッドのパラメーターとしてtype_code.Blobを渡す。
   - SQL実行メソッドで一度使用すると、そのtype_code.Blobは再利用できない。

#### insertの例

```python
    with connection.cursor() as cursor:
        insert_sql = "insert into blob_example values (?, ?)"
        parameters_list = [
            (1, cursor.upload_blob(None)),
            (2, cursor.upload_blob(b"abc")),
            (3, cursor.upload_blob(b"def")),
        ]
        cursor.executemany(insert_sql, parameters_list)
        print("insert rowcount:", cursor.rowcount)
        connection.commit()
```

> [!TIP]
>
> upload_blobメソッドの第2引数でタイムアウト時間を指定することもできます。

### prepareメソッドでBLOBを使用する方法

Tsurugi Python DB-API独自の拡張機能として、SQL実行前にパラメーターの型を指定することができます。
この方法を使うと、パラメーターにバイト列を直接記述できます。

```python
        insert_sql = "insert into blob_example values (?, ?)"
        placeholders = [tsurugi.type_code.Int32, tsurugi.type_code.Blob]
        cursor.prepare(insert_sql, placeholders)

        parameters_list = [
            (1, None),
            (2, b"abc"),
            (3, b"def"),
        ]
        cursor.executemany(insert_sql, parameters_list)  # ここでアップロードされる
        print("insert rowcount:", cursor.rowcount)
        connection.commit()
```

> [!NOTE]
>
> この方法の場合、SQL実行メソッド内でBLOBデータがアップロードされます。

## select文の実行結果からBLOBを取得する方法

select文の実行結果にBLOBが含まれる場合、バイト列（bytes）として取得されます。
（特に特別な操作をする必要はありません）

## BLOB中継サービスのエンドポイント

BLOB中継サービスの接続先URI（エンドポイント）は、Tsurugi DBから送られ、Tsurugi Python DB-API内部で使用しています。

しかし、ネットワーク環境によっては、Tsurugi DB内部で管理しているホスト名やIPアドレスでは、クライアントから接続できないことがあります。

このため、Tsurugi Python DB-APIでBLOB中継サービスのエンドポイントを指定することができるようになっています。

```python
    config = tsurugi.Config()
    config.endpoint = "tcp://localhost:12345"

    config.blob_relay_service_endpoint = "http://localhost:52345"
```

なお、Tsurugi Python DB-APIでは `dns:///` は使用できません。`http://` に置き換えてください。

### CA証明書

HTTPS接続時にサーバー証明書の検証に使用するCA証明書を指定することもできます。

```python
    config.blob_relay_service_endpoint = "https://localhost:52345"
    config.blob_relay_service_ca_cert_pem_file = "/path/to/ca_cert.pem"
```

