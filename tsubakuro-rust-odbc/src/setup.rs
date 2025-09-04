#![allow(clippy::upper_case_acronyms)]

pub mod config_dsn;
pub(crate) mod dsn_dialog_value;
#[cfg(target_os = "windows")]
pub(crate) mod dsn_dialog_windows;
pub(crate) mod installer_api;

type BOOL = i32;
type WORD = u16;
type DWORD = u32;
type LPCSTR = *const u8;
type LPCWSTR = *const u16;
