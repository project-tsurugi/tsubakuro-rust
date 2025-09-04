#![allow(dead_code)]

use log::{debug, warn};

use crate::{
    ctype::SqlReturn,
    setup::*,
    util::{string_to_utf16, utf16_to_string},
};

#[cfg_attr(windows, link(name = "odbccp32", kind = "raw-dylib"))]
#[cfg_attr(unix, link(name = "odbcinst"))]
unsafe extern "system" {
    unsafe fn SQLValidDSNW(dsn: LPCWSTR) -> BOOL;
    unsafe fn SQLWriteDSNToIniW(dsn: LPCWSTR, driver: LPCWSTR) -> BOOL;
    unsafe fn SQLRemoveDSNFromIniW(dsn: LPCWSTR) -> BOOL;
    unsafe fn SQLWritePrivateProfileStringW(
        section: LPCWSTR,
        entry: LPCWSTR,
        value: LPCWSTR,
        filename: LPCWSTR,
    ) -> BOOL;
    unsafe fn SQLInstallerErrorW(
        error: WORD,
        error_code: *mut DWORD,
        error_message: LPCWSTR,
        error_message_len_max: WORD,
        error_message_len: *mut WORD,
    ) -> SqlReturn;
}

pub(crate) fn valid_dsn(dsn: &str) -> Result<(), String> {
    let dsn = string_to_utf16(dsn);
    unsafe {
        if SQLValidDSNW(dsn.as_ptr()) != 0 {
            Ok(())
        } else {
            const ODBC_FUNCTION_NAME: &str = "SQLValidDSNW()";
            Err(get_installer_error_message(ODBC_FUNCTION_NAME))
        }
    }
}

pub(crate) fn write_dsn_to_ini(dsn: &str, driver: &str) -> Result<(), String> {
    let dsn = string_to_utf16(dsn);
    let driver = string_to_utf16(driver);

    unsafe {
        if SQLWriteDSNToIniW(dsn.as_ptr(), driver.as_ptr()) != 0 {
            Ok(())
        } else {
            const ODBC_FUNCTION_NAME: &str = "SQLWriteDSNToIniW()";
            Err(get_installer_error_message(ODBC_FUNCTION_NAME))
        }
    }
}

pub(crate) fn remove_dsn_from_ini(dsn: &str) -> Result<(), String> {
    const FUNCTION_NAME: &str = "remove_dsn_from_ini()";

    let dsn_utf16 = string_to_utf16(dsn);
    unsafe {
        if SQLRemoveDSNFromIniW(dsn_utf16.as_ptr()) != 0 {
            debug!("{FUNCTION_NAME}: SQLRemoveDSNFromIniW({}) succeeded", dsn);
            Ok(())
        } else {
            const ODBC_FUNCTION_NAME: &str = "SQLRemoveDSNFromIniW()";
            Err(get_installer_error_message(ODBC_FUNCTION_NAME))
        }
    }
}

pub(crate) fn write_private_profile_string(
    section: &str,
    entry: &str,
    value: &str,
    filename: &str,
) -> Result<(), String> {
    let section = string_to_utf16(section);
    let entry = string_to_utf16(entry);
    let value = string_to_utf16(value);
    let filename = string_to_utf16(filename);

    unsafe {
        if SQLWritePrivateProfileStringW(
            section.as_ptr(),
            entry.as_ptr(),
            value.as_ptr(),
            filename.as_ptr(),
        ) != 0
        {
            Ok(())
        } else {
            const ODBC_FUNCTION_NAME: &str = "SQLWritePrivateProfileStringW()";
            Err(get_installer_error_message(ODBC_FUNCTION_NAME))
        }
    }
}

fn get_installer_error_message(caller_name: &str) -> String {
    const FUNCTION_NAME: &str = "installer_error()";

    let mut result = Vec::with_capacity(9);
    result.push(format!("{caller_name} error."));

    for error in 1..=8 {
        match installer_error(error, 1024) {
            Ok(Some(installer_error)) => {
                debug!("{caller_name} error[{}]: {:?}", error, installer_error);
                result.push(installer_error.to_string());
            }
            Ok(None) => break,
            Err(e) => {
                warn!("{FUNCTION_NAME} error. {:?}", e);
                break;
            }
        }
    }

    result.join("\r\n")
}

#[derive(Debug)]
pub(crate) struct InstallerError {
    pub error_code: DWORD,
    pub error_message: String,
}

impl std::fmt::Display for InstallerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:08x}: {}", self.error_code, self.error_message)
    }
}

impl std::error::Error for InstallerError {}

fn installer_error(
    error: WORD,
    error_message_len_max: WORD,
) -> Result<Option<InstallerError>, Box<dyn std::error::Error>> {
    const FUNCTION_NAME: &str = "installer_error()";

    let mut error_code = 0u32;
    let mut error_message_len = 0u16;
    let mut error_message = vec![0u16; error_message_len_max as usize];

    let rc = unsafe {
        SQLInstallerErrorW(
            error,
            &mut error_code,
            error_message.as_mut_ptr(),
            error_message_len_max,
            &mut error_message_len,
        )
    };

    use SqlReturn::*;
    match rc {
        SQL_SUCCESS => {
            let error_message = utf16_to_string(&error_message);
            Ok(Some(InstallerError {
                error_code,
                error_message,
            }))
        }
        SQL_SUCCESS_WITH_INFO => installer_error(error, error_message_len + 1),
        SQL_NO_DATA => Ok(None),
        _ => Err(format!("{FUNCTION_NAME} error. {:?}", rc).into()),
    }
}
