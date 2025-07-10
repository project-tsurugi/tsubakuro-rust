use std::{
    ffi::{c_char, CStr, CString},
    sync::Arc,
};

use crate::{
    ctype::{SqlChar, SqlInteger, SqlLen, SqlReturn, SqlSmallInt, SqlWChar},
    handle::diag::{TsurugiOdbcDiagCollection, TsurugiOdbcError},
};

const SQL_NTS: SqlSmallInt = -3;

pub(crate) fn char_to_string(
    src: *const SqlChar,
    length: SqlSmallInt,
) -> Result<String, Box<dyn std::error::Error>> {
    if src.is_null() {
        return Err("char_to_string(): src is null".into());
    }

    if length == SQL_NTS {
        let s = unsafe { CStr::from_ptr(src as *const c_char) };
        s.to_str().map(|s| s.to_string()).map_err(|e| e.into())
    } else {
        let slice = unsafe { std::slice::from_raw_parts(src, length as usize) };
        let end = slice.iter().position(|&c| c == 0).unwrap_or(slice.len());
        std::str::from_utf8(&slice[..end])
            .map(|s| s.to_string())
            .map_err(|e| e.into())
    }
}

pub(crate) fn char_to_string_integer(
    src: *const SqlChar,
    length: SqlInteger,
) -> Result<String, Box<dyn std::error::Error>> {
    if src.is_null() {
        return Err("char_to_string_integer(): src is null".into());
    }

    if length == SQL_NTS.into() {
        let s = unsafe { CStr::from_ptr(src as *const c_char) };
        s.to_str().map(|s| s.to_string()).map_err(|e| e.into())
    } else {
        let slice = unsafe { std::slice::from_raw_parts(src, length as usize) };
        let end = slice.iter().position(|&c| c == 0).unwrap_or(slice.len());
        std::str::from_utf8(&slice[..end])
            .map(|s| s.to_string())
            .map_err(|e| e.into())
    }
}

pub(crate) fn wchar_to_string(
    src: *const SqlWChar,
    length: SqlSmallInt,
) -> Result<String, Box<dyn std::error::Error>> {
    if src.is_null() {
        return Err("wchar_to_string(): src is null".into());
    }

    if length == SQL_NTS {
        let mut len = 0;
        unsafe {
            while *src.add(len) != 0 {
                len += 1;
            }
            let slice = std::slice::from_raw_parts(src, len);
            String::from_utf16(slice).map_err(|e| e.into())
        }
    } else {
        let slice = unsafe { std::slice::from_raw_parts(src, length as usize) };
        let end = slice.iter().position(|&c| c == 0).unwrap_or(slice.len());
        String::from_utf16(&slice[..end]).map_err(|e| e.into())
    }
}

pub(crate) fn wchar_to_string_integer(
    src: *const SqlWChar,
    length: SqlInteger,
) -> Result<String, Box<dyn std::error::Error>> {
    if src.is_null() {
        return Err("wchar_to_string_integer(): src is null".into());
    }

    if length == SQL_NTS.into() {
        let mut len = 0;
        unsafe {
            while *src.add(len) != 0 {
                len += 1;
            }
            let slice = std::slice::from_raw_parts(src, len);
            String::from_utf16(slice).map_err(|e| e.into())
        }
    } else {
        let slice = unsafe { std::slice::from_raw_parts(src, length as usize) };
        let end = slice.iter().position(|&c| c == 0).unwrap_or(slice.len());
        String::from_utf16(&slice[..end]).map_err(|e| e.into())
    }
}

pub(crate) fn char_to_string_opt(
    src: *const SqlChar,
    length: SqlSmallInt,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    if src.is_null() {
        return Ok(None);
    }

    if length == SQL_NTS {
        let s = unsafe { CStr::from_ptr(src as *const c_char) };
        s.to_str()
            .map(|s| Some(s.to_string()))
            .map_err(|e| e.into())
    } else {
        let slice = unsafe { std::slice::from_raw_parts(src, length as usize) };
        let end = slice.iter().position(|&c| c == 0).unwrap_or(slice.len());
        std::str::from_utf8(&slice[..end])
            .map(|s| Some(s.to_string()))
            .map_err(|e| e.into())
    }
}

pub(crate) fn wchar_to_string_opt(
    src: *const SqlWChar,
    length: SqlSmallInt,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    if src.is_null() {
        return Ok(None);
    }

    if length == SQL_NTS {
        let mut len = 0;
        unsafe {
            while *src.add(len) != 0 {
                len += 1;
            }
            let slice = std::slice::from_raw_parts(src, len);
            String::from_utf16(slice)
                .map(|s| Some(s.to_string()))
                .map_err(|e| e.into())
        }
    } else {
        let slice = unsafe { std::slice::from_raw_parts(src, length as usize) };
        let end = slice.iter().position(|&c| c == 0).unwrap_or(slice.len());
        String::from_utf16(&slice[..end])
            .map(|s| Some(s.to_string()))
            .map_err(|e| e.into())
    }
}

pub(crate) fn write_char(
    name: &str,
    src: &str,
    dst: *mut SqlChar,
    buffer_length: SqlSmallInt,
    out_length: *mut SqlSmallInt,
    diags: Option<&Arc<TsurugiOdbcDiagCollection>>,
) -> SqlReturn {
    let (rc, len) = write_char0(name, src, dst, buffer_length as usize, diags);

    if !out_length.is_null() {
        unsafe {
            *out_length = len as SqlSmallInt;
        }
    }

    rc
}

pub(crate) fn write_char_len(
    name: &str,
    src: &str,
    dst: *mut SqlChar,
    buffer_length: SqlLen,
    out_length: *mut SqlLen,
    diags: &Arc<TsurugiOdbcDiagCollection>,
) -> SqlReturn {
    let (rc, len) = write_char0(name, src, dst, buffer_length as usize, Some(diags));

    if !out_length.is_null() {
        unsafe {
            *out_length = len as SqlLen;
        }
    }

    rc
}
fn write_char0(
    name: &str,
    src: &str,
    dst: *mut SqlChar,
    buffer_length: usize,
    diags: Option<&Arc<TsurugiOdbcDiagCollection>>,
) -> (SqlReturn, usize) {
    let s = CString::new(src).unwrap();
    let bytes = s.as_bytes_with_nul();
    let value_len = bytes.len();

    let rc = if dst.is_null() {
        SqlReturn::SQL_SUCCESS
    } else {
        let copy_len = value_len.min(buffer_length);
        unsafe {
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), dst, copy_len);
        }

        if value_len <= buffer_length {
            SqlReturn::SQL_SUCCESS
        } else {
            if let Some(diags) = diags {
                diags.add_diag(TsurugiOdbcError::DataTruncated, format!("{name} truncated"));
            } else {
                // do nothing for SQLDiagRec
            }
            SqlReturn::SQL_SUCCESS_WITH_INFO
        }
    };

    (rc, value_len - 1)
}

pub(crate) fn write_wchar(
    name: &str,
    src: &str,
    dst: *mut SqlWChar,
    buffer_length: SqlSmallInt,
    out_length: *mut SqlSmallInt,
    diags: Option<&Arc<TsurugiOdbcDiagCollection>>,
) -> SqlReturn {
    let buffer_chars = buffer_length as usize;
    let (rc, len) = write_wchar0(name, src, dst, buffer_chars, diags);

    if !out_length.is_null() {
        unsafe {
            *out_length = len as SqlSmallInt;
        }
    }

    rc
}

pub(crate) fn write_wchar_bytes(
    name: &str,
    src: &str,
    dst: *mut SqlWChar,
    buffer_length: SqlSmallInt,
    out_length: *mut SqlSmallInt,
    diags: &Arc<TsurugiOdbcDiagCollection>,
) -> SqlReturn {
    let buffer_chars = buffer_length as usize / 2; // SqlWChar is 2 bytes
    let (rc, len) = write_wchar0(name, src, dst, buffer_chars, Some(diags));

    if !out_length.is_null() {
        unsafe {
            *out_length = (len * 2) as SqlSmallInt;
        }
    }

    rc
}

pub(crate) fn write_wchar_len_bytes(
    name: &str,
    src: &str,
    dst: *mut SqlWChar,
    buffer_length: SqlLen,
    out_length: *mut SqlLen,
    diags: &Arc<TsurugiOdbcDiagCollection>,
) -> SqlReturn {
    let buffer_chars = buffer_length as usize / 2; // SqlWChar is 2 bytes
    let (rc, len) = write_wchar0(name, src, dst, buffer_chars, Some(diags));

    if !out_length.is_null() {
        unsafe {
            *out_length = (len * 2) as SqlLen;
        }
    }

    rc
}

fn write_wchar0(
    name: &str,
    src: &str,
    dst: *mut SqlWChar,
    buffer_chars: usize,
    diags: Option<&Arc<TsurugiOdbcDiagCollection>>,
) -> (SqlReturn, usize) {
    let mut utf16 = src.encode_utf16().collect::<Vec<u16>>();
    utf16.push(0); // nul-terminate
    let value_len = utf16.len();

    let rc = if dst.is_null() {
        SqlReturn::SQL_SUCCESS
    } else {
        let copy_len = value_len.min(buffer_chars);
        unsafe {
            std::ptr::copy_nonoverlapping(utf16.as_ptr(), dst, copy_len);
        }

        if value_len <= buffer_chars {
            SqlReturn::SQL_SUCCESS
        } else {
            if let Some(diags) = diags {
                diags.add_diag(TsurugiOdbcError::DataTruncated, format!("{name} truncated"));
            } else {
                // do nothing for SQLDiagRecW
            }
            SqlReturn::SQL_SUCCESS_WITH_INFO
        }
    };

    (rc, value_len - 1)
}
