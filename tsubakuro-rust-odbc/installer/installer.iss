; Tsurugi ODBC Driver installer

#define MyAppName "Tsurugi ODBC Driver"
#define MyAppVersion "0.8.0"
#define MyAppPublisher "Project Tsurugi"
#define MyAppURL "https://github.com/project-tsurugi/tsurugidb"
#define MyAppDllName "tsubakuro_rust_odbc.dll"

[Setup]
AppId={{975F2B09-F639-4721-A3B1-76FA3B8B5259}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppPublisher={#MyAppPublisher}
AppPublisherURL={#MyAppURL}
DefaultDirName={autopf}\Tsurugi\ODBC
DisableDirPage=auto
AlwaysShowDirOnReadyPage=yes
UninstallDisplayIcon={app}\{#MyAppDllName}
; "ArchitecturesAllowed=x64compatible" specifies that Setup cannot run
; on anything but x64 and Windows 11 on Arm.
ArchitecturesAllowed=x64compatible
; "ArchitecturesInstallIn64BitMode=x64compatible" requests that the
; install be done in "64-bit mode" on x64 or Windows 11 on Arm,
; meaning it should use the native 64-bit Program Files directory and
; the 64-bit view of the registry.
ArchitecturesInstallIn64BitMode=x64compatible
DisableWelcomePage=no
LicenseFile=..\..\LICENSE
; Uncomment the following line to run in non administrative install mode (install for current user only).
;PrivilegesRequired=lowest
OutputBaseFilename=tsurugi_odbc_driver_installer
SolidCompression=yes
WizardStyle=modern

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"
Name: "armenian"; MessagesFile: "compiler:Languages\Armenian.isl"
Name: "brazilianportuguese"; MessagesFile: "compiler:Languages\BrazilianPortuguese.isl"
Name: "catalan"; MessagesFile: "compiler:Languages\Catalan.isl"
Name: "corsican"; MessagesFile: "compiler:Languages\Corsican.isl"
Name: "czech"; MessagesFile: "compiler:Languages\Czech.isl"
Name: "danish"; MessagesFile: "compiler:Languages\Danish.isl"
Name: "dutch"; MessagesFile: "compiler:Languages\Dutch.isl"
Name: "finnish"; MessagesFile: "compiler:Languages\Finnish.isl"
Name: "french"; MessagesFile: "compiler:Languages\French.isl"
Name: "german"; MessagesFile: "compiler:Languages\German.isl"
Name: "hebrew"; MessagesFile: "compiler:Languages\Hebrew.isl"
Name: "italian"; MessagesFile: "compiler:Languages\Italian.isl"
Name: "japanese"; MessagesFile: "compiler:Languages\Japanese.isl"
Name: "norwegian"; MessagesFile: "compiler:Languages\Norwegian.isl"
Name: "polish"; MessagesFile: "compiler:Languages\Polish.isl"
Name: "portuguese"; MessagesFile: "compiler:Languages\Portuguese.isl"
Name: "russian"; MessagesFile: "compiler:Languages\Russian.isl"
Name: "slovenian"; MessagesFile: "compiler:Languages\Slovenian.isl"
Name: "spanish"; MessagesFile: "compiler:Languages\Spanish.isl"
Name: "turkish"; MessagesFile: "compiler:Languages\Turkish.isl"
Name: "ukrainian"; MessagesFile: "compiler:Languages\Ukrainian.isl"

[Files]
Source: "..\target\release\{#MyAppDllName}"; DestDir: "{app}"; Flags: ignoreversion

[Registry]
Root: HKLM; Subkey: "SOFTWARE\ODBC\ODBCINST.INI\Tsurugi Driver"; Flags: uninsdeletekeyifempty
Root: HKLM; Subkey: "SOFTWARE\ODBC\ODBCINST.INI\Tsurugi Driver"; ValueType: string; ValueName: "APILevel"; ValueData: "1"; Flags: uninsdeletevalue
Root: HKLM; Subkey: "SOFTWARE\ODBC\ODBCINST.INI\Tsurugi Driver"; ValueType: string; ValueName: "ConnectFunctions"; ValueData: "YYN"; Flags: uninsdeletevalue
Root: HKLM; Subkey: "SOFTWARE\ODBC\ODBCINST.INI\Tsurugi Driver"; ValueType: string; ValueName: "Driver"; ValueData: "{app}\{#MyAppDllName}"; Flags: uninsdeletevalue
Root: HKLM; Subkey: "SOFTWARE\ODBC\ODBCINST.INI\Tsurugi Driver"; ValueType: string; ValueName: "DriverODBCVer"; ValueData: "03.51"; Flags: uninsdeletevalue
Root: HKLM; Subkey: "SOFTWARE\ODBC\ODBCINST.INI\Tsurugi Driver"; ValueType: string; ValueName: "FileUsage"; ValueData: "0"; Flags: uninsdeletevalue
Root: HKLM; Subkey: "SOFTWARE\ODBC\ODBCINST.INI\Tsurugi Driver"; ValueType: string; ValueName: "Setup"; ValueData: "{app}\{#MyAppDllName}"; Flags: uninsdeletevalue
Root: HKLM; Subkey: "SOFTWARE\ODBC\ODBCINST.INI\Tsurugi Driver"; ValueType: string; ValueName: "SQLLevel"; ValueData: "0"; Flags: uninsdeletevalue
Root: HKLM; Subkey: "SOFTWARE\ODBC\ODBCINST.INI\Tsurugi Driver"; ValueType: dword; ValueName: "UsageCount"; ValueData: $00000001; Flags: uninsdeletevalue
Root: HKLM; Subkey: "SOFTWARE\ODBC\ODBCINST.INI\ODBC Drivers"; ValueType: string; ValueName: "Tsurugi Driver"; ValueData: "Installed"; Flags: uninsdeletevalue
