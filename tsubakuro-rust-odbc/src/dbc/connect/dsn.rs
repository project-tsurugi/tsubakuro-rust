use crate::{
    dbc::connect::{connect_tsurugi::TsurugiOdbcConnectArguments, connection_string::ENDPOINT},
    util::string_to_utf16,
};

type WChar = u16;

#[cfg_attr(windows, link(name = "odbccp32", kind = "raw-dylib"))]
#[cfg_attr(unix, link(name = "odbcinst"))]
unsafe extern "system" {
    unsafe fn SQLGetPrivateProfileStringW(
        section: *const WChar,
        entry: *const WChar,
        default: *const WChar,
        return_buffer: *mut WChar,
        return_buffer_length: i32,
        file_name: *const WChar,
    ) -> i32;
}

#[cfg(windows)]
pub(crate) const FILE_NAME: &str = "odbc.ini";

#[cfg(unix)]
pub(crate) const FILE_NAME: &str = ".odbc.ini";

pub(crate) fn read_dsn(dsn: &str) -> TsurugiOdbcConnectArguments {
    let mut arg = TsurugiOdbcConnectArguments::new();
    arg.dsn = Some(dsn.into());

    if let Ok(value) = get_private_profile_string(dsn, ENDPOINT, "") {
        if !value.is_empty() {
            arg.endpoint = Some(value);
        }
    }

    arg
}

pub(crate) fn get_private_profile_string(
    section: &str,
    entry: &str,
    default: &str,
) -> Result<String, i32> {
    let section = string_to_utf16(section);
    let entry = string_to_utf16(entry);
    let default = string_to_utf16(default);
    let file_name = string_to_utf16(FILE_NAME);

    let mut return_buffer = vec![0; 1024];
    let result = unsafe {
        SQLGetPrivateProfileStringW(
            section.as_ptr(),
            entry.as_ptr(),
            default.as_ptr(),
            return_buffer.as_mut_ptr(),
            return_buffer.len() as i32,
            file_name.as_ptr(),
        )
    };

    if result == 0 {
        Ok("".into())
    } else if result > 0 {
        let value = String::from_utf16_lossy(&return_buffer[..result as usize]);
        Ok(value)
    } else {
        Err(result)
    }
}
