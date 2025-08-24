# Tsurugi ODBC Driver Installer

Tsurugi ODBC Driver installer for MS-Windows.

## How to build

Since [Inno Setup 6](https://jrsoftware.org/isinfo.php) is used, it must be installed.

```dos
cd tsubakuro-rust-odbc\installer
"C:\Program Files (x86)\Inno Setup 6\ISCC.exe" installer.iss
dir Output\
```

