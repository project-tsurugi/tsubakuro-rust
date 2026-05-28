# tsubakuro-rust-core 変更点

## 0.10.0

- LOB転送モードのデフォルトが 特権モードからBLOB中継サービス利用モードに変更。
- BLOB, CLOBをプリペアードステートメントのパラメーターに指定する方法が変更。
- Jobのジェネリクスが `T` から `T: Send + 'static` に変更。

### BLOB, CLOBのプリペアードステートメントのパラメーター

0.10.0より前は、以下のようにしてBLOBのプリペアードステートメントのパラメーターを指定していたが、非推奨になった。

```rust
let blob = TgBlob::new("/path/to/file");
let paraemter = SqlParameter.of("name", blob);
```

0.10.0では、BLOBファイルを使用する場合は以下のようにする。

```rust
let blob = sql_client.upload_blob_file("/path/to/file").await?;
let paraemter = SqlParameter.of("name", blob);
```

