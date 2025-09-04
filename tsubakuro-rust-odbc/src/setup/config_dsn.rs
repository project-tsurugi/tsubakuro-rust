use log::{debug, trace, warn};

use crate::{
    ctype::HWnd,
    dbc::connect::{
        connect_tsurugi::TsurugiOdbcCredentialType,
        connection_string::{
            ConnectionAttributes, KEY_AUTH_TOKEN, KEY_CREDENTIALS, KEY_CREDENTIAL_TYPE,
            KEY_ENDPOINT, KEY_PASSWORD, KEY_USER,
        },
        dsn::FILE_NAME,
    },
    logger::env_logger_init,
    setup::{
        dsn_dialog_value::{DsnDialogCredentialRadio, DsnDialogValue},
        installer_api::{remove_dsn_from_ini, write_dsn_to_ini, write_private_profile_string},
        *,
    },
    util::{utf16_to_string, utf8_to_string},
};

#[no_mangle]
pub extern "system" fn ConfigDSN(
    parent_hwnd: HWnd,
    request: WORD,
    driver: LPCSTR,
    attributes: LPCSTR,
) -> BOOL {
    env_logger_init();
    const FUNCTION_NAME: &str = "ConfigDSN()";
    trace!(
        "{FUNCTION_NAME} start. parent_hwnd={:?}, request={:?}, driver={:?}, attributes={:?}",
        parent_hwnd,
        request,
        driver,
        attributes
    );

    let driver = lpcstr_to_string(driver);
    let attributes = parse_attributes(attributes);
    let rc = config_dsn(parent_hwnd, request, driver, attributes);

    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc.into()
}

fn lpcstr_to_string(value: LPCSTR) -> Result<Option<String>, Box<dyn std::error::Error>> {
    if value.is_null() {
        return Ok(None);
    }

    let v = unsafe {
        let mut len = 0;
        while *value.add(len) != 0 {
            len += 1;
        }
        std::slice::from_raw_parts(value, len)
    };
    Ok(Some(utf8_to_string(v)))
}

fn parse_attributes(attributes: LPCSTR) -> ConnectionAttributes {
    let mut result = ConnectionAttributes::new();
    if attributes.is_null() {
        return result;
    }

    unsafe {
        let mut first = true;
        let mut key_start = 0;
        let mut key_end = 0;
        let mut value_start = 0;
        let mut value_started = false;
        let mut i = 0;
        loop {
            let c = *attributes.add(i);
            if c == 0 {
                if first {
                    break;
                }
                if value_started {
                    let value_end = i;
                    let key = utf8_to_string(std::slice::from_raw_parts(
                        attributes.add(key_start),
                        key_end - key_start,
                    ));
                    let value = utf8_to_string(std::slice::from_raw_parts(
                        attributes.add(value_start),
                        value_end - value_start,
                    ));
                    result.set(&key, value);
                }

                first = true;
                i += 1;
                key_start = i;
                value_started = false;
                continue;
            }
            if c == b'=' {
                #[allow(clippy::collapsible_if)]
                if !value_started {
                    value_started = true;
                    key_end = i;
                    value_start = i + 1;
                }
            }

            first = false;
            i += 1;
        }
    }
    result
}

#[no_mangle]
pub extern "system" fn ConfigDSNW(
    parent_hwnd: HWnd,
    request: WORD,
    driver: LPCWSTR,
    attributes: LPCWSTR,
) -> BOOL {
    env_logger_init();
    const FUNCTION_NAME: &str = "ConfigDSNW()";
    trace!(
        "{FUNCTION_NAME} start. parent_hwnd={:?}, request={:?}, driver={:?}, attributes={:?}",
        parent_hwnd,
        request,
        driver,
        attributes
    );

    let driver = lpcwstr_to_string(driver);
    let attributes = parse_attributesw(attributes);
    let rc = config_dsn(parent_hwnd, request, driver, attributes);

    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc.into()
}

fn lpcwstr_to_string(value: LPCWSTR) -> Result<Option<String>, Box<dyn std::error::Error>> {
    if value.is_null() {
        return Ok(None);
    }

    let v = unsafe {
        let mut len = 0;
        while *value.add(len) != 0 {
            len += 1;
        }
        std::slice::from_raw_parts(value, len)
    };
    Ok(Some(utf16_to_string(v)))
}

fn parse_attributesw(attributes: LPCWSTR) -> ConnectionAttributes {
    let mut result = ConnectionAttributes::new();
    if attributes.is_null() {
        return result;
    }

    unsafe {
        let mut first = true;
        let mut key_start = 0;
        let mut key_end = 0;
        let mut value_start = 0;
        let mut value_started = false;
        let mut i = 0;
        loop {
            let c = *attributes.add(i);
            if c == 0 {
                if first {
                    break;
                }
                if value_started {
                    let value_end = i;
                    let key = utf16_to_string(std::slice::from_raw_parts(
                        attributes.add(key_start),
                        key_end - key_start,
                    ));
                    let value = utf16_to_string(std::slice::from_raw_parts(
                        attributes.add(value_start),
                        value_end - value_start,
                    ));
                    result.set(&key, value);
                }

                first = true;
                i += 1;
                key_start = i;
                value_started = false;
                continue;
            }
            if c == '=' as u16 {
                #[allow(clippy::collapsible_if)]
                if !value_started {
                    value_started = true;
                    key_end = i;
                    value_start = i + 1;
                }
            }

            first = false;
            i += 1;
        }
    }
    result
}

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub(crate) enum RequestType {
    ODBC_ADD_DSN = 1,            // Add data source
    ODBC_CONFIG_DSN = 2,         // Configure (edit) data source
    ODBC_REMOVE_DSN = 3,         // Remove data source
    ODBC_ADD_SYS_DSN = 4,        // add a system DSN
    ODBC_CONFIG_SYS_DSN = 5,     // Configure a system DSN
    ODBC_REMOVE_SYS_DSN = 6,     // remove a system DSN
    ODBC_REMOVE_DEFAULT_DSN = 7, // remove the default DSN
}

impl TryFrom<u16> for RequestType {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        use RequestType::*;
        match value {
            1 => Ok(ODBC_ADD_DSN),
            2 => Ok(ODBC_CONFIG_DSN),
            3 => Ok(ODBC_REMOVE_DSN),
            4 => Ok(ODBC_ADD_SYS_DSN),
            5 => Ok(ODBC_CONFIG_SYS_DSN),
            6 => Ok(ODBC_REMOVE_SYS_DSN),
            7 => Ok(ODBC_REMOVE_DEFAULT_DSN),
            e => Err(e),
        }
    }
}

impl RequestType {
    fn is_add(&self) -> bool {
        use RequestType::*;
        matches!(self, ODBC_ADD_DSN | ODBC_ADD_SYS_DSN)
    }
}

fn config_dsn(
    parent_hwnd: HWnd,
    request: WORD,
    driver: Result<Option<String>, Box<dyn std::error::Error>>,
    attributes: ConnectionAttributes,
) -> bool {
    const FUNCTION_NAME: &str = "config_dsn()";

    let request = match RequestType::try_from(request) {
        Ok(value) => value,
        Err(e) => {
            debug!("{FUNCTION_NAME} error. Unsupported request type {}", e);
            return false;
        }
    };
    let driver = match driver {
        Ok(Some(value)) => value,
        Ok(None) => {
            debug!("{FUNCTION_NAME} error. Driver name is null");
            return false;
        }
        Err(e) => {
            debug!(
                "{FUNCTION_NAME} error. Failed to convert driver name. {}",
                e
            );
            return false;
        }
    };

    debug!(
        "{FUNCTION_NAME}: request={:?}, driver={:?}, attributes={:?}",
        request, driver, attributes
    );

    use RequestType::*;
    match request {
        ODBC_REMOVE_DSN | ODBC_REMOVE_SYS_DSN | ODBC_REMOVE_DEFAULT_DSN => remove_dsn(attributes),
        _ => update_dsn(parent_hwnd, request, driver, attributes),
    }
}

fn update_dsn(
    parent_hwnd: HWnd,
    request: RequestType,
    driver: String,
    attributes: ConnectionAttributes,
) -> bool {
    const FUNCTION_NAME: &str = "update_dsn()";

    let dialog_value = DsnDialogValue::from(attributes, request.is_add());

    let dialog_value = if parent_hwnd.is_null() {
        debug!("{FUNCTION_NAME}: dialog_value={:?}", dialog_value);
        dialog_value
    } else {
        #[cfg(target_os = "windows")]
        {
            debug!("{FUNCTION_NAME}: before dialog_value={:?}", dialog_value);

            use super::dsn_dialog_windows::dsn_dialog;
            let dialog_value = match dsn_dialog(parent_hwnd, dialog_value) {
                Ok(Some(value)) => value,
                Ok(None) => return true, // Cancelled by user
                Err(_) => return false,
            };

            debug!("{FUNCTION_NAME}: after  dialog_value={:?}", dialog_value);
            dialog_value
        }
        #[cfg(not(target_os = "windows"))]
        {
            debug!("{FUNCTION_NAME}: dialog_value={:?}", dialog_value);
            dialog_value
        }
    };

    if dialog_value.is_new_dsn() {
        #[allow(clippy::collapsible_if)]
        if write_dsn_to_ini(dialog_value.data_source_name(), &driver).is_err() {
            return false;
        }
    }

    let section = dialog_value.data_source_name();

    let mut rc = true;
    rc &= write_profile(section, KEY_ENDPOINT, dialog_value.endpoint());
    rc &= write_profile(section, KEY_USER, dialog_value.user());
    rc &= write_profile(section, KEY_PASSWORD, dialog_value.password());
    rc &= write_profile(section, KEY_AUTH_TOKEN, dialog_value.auth_token());
    rc &= write_profile(section, KEY_CREDENTIALS, dialog_value.credential_file());
    let credential_type = credential_type(dialog_value.credential_radio());
    rc &= write_profile(section, KEY_CREDENTIAL_TYPE, &credential_type);

    if dialog_value.is_rename_dsn() {
        if let Some(old_dsn) = dialog_value.dsn() {
            rc &= remove_dsn_from_ini(old_dsn).is_ok();
        }
    }

    rc
}

fn credential_type(radio: DsnDialogCredentialRadio) -> String {
    use DsnDialogCredentialRadio::*;
    let credential_type = match radio {
        Nothing | Null => TsurugiOdbcCredentialType::Null,
        UserPassword => TsurugiOdbcCredentialType::UserPassword,
        AuthToken => TsurugiOdbcCredentialType::AuthToken,
        File => TsurugiOdbcCredentialType::File,
    };
    (credential_type as i32).to_string()
}

fn write_profile(section: &str, entry: &str, value: &str) -> bool {
    write_private_profile_string(section, entry, value, FILE_NAME).is_ok()
}

fn remove_dsn(attributes: ConnectionAttributes) -> bool {
    const FUNCTION_NAME: &str = "remove_dsn()";

    let dsn = match attributes.dsn() {
        Some(value) => value,
        None => {
            warn!("{FUNCTION_NAME} error. DSN not found in attributes");
            return false;
        }
    };

    remove_dsn_from_ini(dsn).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_attributes() {
        let s = b"\0\0";
        let attributes = parse_attributes(s.as_ptr() as LPCSTR);
        assert_eq!(None, attributes.dsn());
        assert_eq!(None, attributes.endpoint());

        let s = b"key1=value1\0key2=value=2\0\0";
        let attributes = parse_attributes(s.as_ptr() as LPCSTR);
        assert_eq!("value1", attributes.get("key1").unwrap());
        assert_eq!("value=2", attributes.get("key2").unwrap());
    }

    #[test]
    fn test_parse_attributesw() {
        let s = "\0\0".encode_utf16().collect::<Vec<u16>>();
        let attributes = parse_attributesw(s.as_ptr() as LPCWSTR);
        assert_eq!(None, attributes.dsn());
        assert_eq!(None, attributes.endpoint());

        let s = "key1=value1\0key2=value=2\0\0"
            .encode_utf16()
            .collect::<Vec<u16>>();
        let attributes = parse_attributesw(s.as_ptr() as LPCWSTR);
        assert_eq!("value1", attributes.get("key1").unwrap());
        assert_eq!("value=2", attributes.get("key2").unwrap());
    }
}
