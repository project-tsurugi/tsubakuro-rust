use crate::{
    dbc::connect::{
        connect_tsurugi::{TsurugiOdbcConnectArguments, TsurugiOdbcCredentialType},
        connection_string::{
            KEY_AUTH_TOKEN, KEY_CREDENTIALS, KEY_CREDENTIAL_TYPE, KEY_ENDPOINT, KEY_PASSWORD,
            KEY_USER,
        },
    },
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
    arg.set_dsn(dsn.into());

    if let Some(value) = get_dsn_entry(dsn, KEY_ENDPOINT) {
        arg.set_endpoint(value);
    }
    if let Some(value) = get_dsn_entry(dsn, KEY_USER) {
        arg.set_user(value);
    }
    if let Some(value) = get_dsn_entry(dsn, KEY_PASSWORD) {
        arg.set_password(value);
    }
    if let Some(value) = get_dsn_entry(dsn, KEY_AUTH_TOKEN) {
        arg.set_auth_token(value);
    }
    if let Some(value) = get_dsn_entry(dsn, KEY_CREDENTIALS) {
        arg.set_credentials(value);
    }
    arg.set_credential_type(credential_type(dsn, &arg));

    arg
}

fn credential_type(section: &str, arg: &TsurugiOdbcConnectArguments) -> TsurugiOdbcCredentialType {
    if let Some(credential_type) = get_dsn_entry(section, KEY_CREDENTIAL_TYPE) {
        if let Ok(value) = TsurugiOdbcCredentialType::try_from(credential_type.as_str()) {
            return value;
        }
    }

    if arg.user().is_some() {
        return TsurugiOdbcCredentialType::UserPassword;
    }
    if arg.auth_token().is_some() {
        return TsurugiOdbcCredentialType::AuthToken;
    }
    if arg.credentials().is_some() {
        return TsurugiOdbcCredentialType::File;
    }

    TsurugiOdbcCredentialType::Null
}

fn get_dsn_entry(section: &str, entry: &str) -> Option<String> {
    if let Ok(value) = get_private_profile_string(section, entry, "") {
        if !value.is_empty() {
            return Some(value);
        }
    }
    None
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
