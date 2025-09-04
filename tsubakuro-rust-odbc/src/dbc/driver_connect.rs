use std::{ffi::c_void, sync::Arc};

use log::{debug, trace};

use crate::{
    check_dbc,
    ctype::{HWnd, SqlChar, SqlReturn, SqlSmallInt, SqlUSmallInt, SqlWChar},
    dbc::connect::{
        connect_tsurugi::{
            connect_tsurugi, TsurugiOdbcConnectArguments, TsurugiOdbcCredentialType,
        },
        connection_string::{
            ConnectionAttributes, KEY_AUTH_TOKEN, KEY_CREDENTIALS, KEY_DRIVER, KEY_DSN,
            KEY_ENDPOINT, KEY_PASSWORD, KEY_USER,
        },
        dsn::read_dsn,
    },
    handle::{
        diag::TsurugiOdbcError,
        hdbc::{HDbc, TsurugiOdbcDbc},
    },
    setup::dsn_dialog_value::DsnDialogValue,
    util::{char_to_string, wchar_to_string, write_char, write_wchar},
};

#[repr(u16)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
enum DriverCompletion {
    SQL_DRIVER_NOPROMPT = 0,
    SQL_DRIVER_COMPLETE = 1,
    SQL_DRIVER_PROMPT = 2,
    SQL_DRIVER_COMPLETE_REQUIRED = 3,
}

impl TryFrom<SqlUSmallInt> for DriverCompletion {
    type Error = SqlUSmallInt;

    fn try_from(value: SqlUSmallInt) -> Result<Self, Self::Error> {
        use DriverCompletion::*;
        match value {
            0 => Ok(SQL_DRIVER_NOPROMPT),
            1 => Ok(SQL_DRIVER_COMPLETE),
            2 => Ok(SQL_DRIVER_PROMPT),
            3 => Ok(SQL_DRIVER_COMPLETE_REQUIRED),
            _ => Err(value),
        }
    }
}

#[no_mangle]
pub extern "system" fn SQLDriverConnect(
    hdbc: HDbc,
    hwnd: HWnd,
    in_connection_string: *const SqlChar,
    in_connection_string_length: SqlSmallInt,
    out_connection_string: *mut SqlChar,
    out_connection_string_size: SqlSmallInt,
    out_connection_string_length: *mut SqlSmallInt,
    driver_completion: SqlUSmallInt,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "SQLDriverConnect()";
    trace!(
        "{FUNCTION_NAME} start. hdbc={:?}, hwnd={:?}, in_connection_string={:?}, in_connection_string_length={:?}, out_connection_string={:?}, out_connection_string_size={:?}, out_connection_string_length={:?}, driver_completion={:?}",
        hdbc, hwnd, in_connection_string, in_connection_string_length, out_connection_string, out_connection_string_size, out_connection_string_length, driver_completion
    );

    let dbc = check_dbc!(hdbc);

    let connection_string = match char_to_string(in_connection_string, in_connection_string_length)
    {
        Ok(s) => s,
        Err(e) => {
            debug!("{FUNCTION_NAME}: connection_string convert error. {:?}", e);
            dbc.add_diag(
                TsurugiOdbcError::StringError,
                format!("connection_string error. {}", e),
            );
            let rc = SqlReturn::SQL_ERROR;
            trace!("{FUNCTION_NAME} end. rc={:?}", rc);
            return rc;
        }
    };

    let delegator = DriverConnect::new(
        dbc,
        hwnd,
        out_connection_string as *mut c_void,
        out_connection_string_size,
        out_connection_string_length,
        driver_completion,
        false,
    );
    let rc = delegator.driver_connect(connection_string);

    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc
}

#[no_mangle]
pub extern "system" fn SQLDriverConnectW(
    hdbc: HDbc,
    hwnd: HWnd,
    in_connection_string: *const SqlWChar,
    in_connection_string_length: SqlSmallInt,
    out_connection_string: *mut SqlWChar,
    out_connection_string_size: SqlSmallInt,
    out_connection_string_length: *mut SqlSmallInt,
    driver_completion: SqlUSmallInt,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "SQLDriverConnectW()";
    trace!(
        "{FUNCTION_NAME} start. hdbc={:?}, hwnd={:?}, in_connection_string={:?}, in_connection_string_length={:?}, out_connection_string={:?}, out_connection_string_size={:?}, out_connection_string_length={:?}, driver_completion={:?}",
        hdbc, hwnd, in_connection_string, in_connection_string_length, out_connection_string, out_connection_string_size, out_connection_string_length, driver_completion
    );

    let dbc = check_dbc!(hdbc);

    let connection_string = match wchar_to_string(in_connection_string, in_connection_string_length)
    {
        Ok(s) => s,
        Err(e) => {
            debug!(
                "{dbc}.{FUNCTION_NAME}: in_connection_string convert error. {:?}",
                e
            );
            dbc.add_diag(
                TsurugiOdbcError::StringError,
                format!("in_connection_string error. {}", e),
            );
            let rc = SqlReturn::SQL_ERROR;
            trace!("{FUNCTION_NAME} end. rc={:?}", rc);
            return rc;
        }
    };

    let delegator = DriverConnect::new(
        dbc,
        hwnd,
        out_connection_string as *mut c_void,
        out_connection_string_size,
        out_connection_string_length,
        driver_completion,
        true,
    );
    let rc = delegator.driver_connect(connection_string);

    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc
}

struct DriverConnect {
    dbc: Arc<TsurugiOdbcDbc>,
    hwnd: HWnd,
    out_connection_string: *mut c_void,
    out_connection_string_size: SqlSmallInt,
    out_connection_string_length: *mut SqlSmallInt,
    driver_completion: SqlUSmallInt,
    wide_char: bool,
}

impl DriverConnect {
    fn new(
        dbc: Arc<TsurugiOdbcDbc>,
        hwnd: HWnd,
        out_connection_string: *mut c_void,
        out_connection_string_size: SqlSmallInt,
        out_connection_string_length: *mut SqlSmallInt,
        driver_completion: SqlUSmallInt,
        wide_char: bool,
    ) -> Self {
        Self {
            dbc,
            hwnd,
            out_connection_string,
            out_connection_string_size,
            out_connection_string_length,
            driver_completion,
            wide_char,
        }
    }

    fn odbc_function_name(&self) -> &'static str {
        if self.wide_char {
            "SQLDriverConnectW()"
        } else {
            "SQLDriverConnect()"
        }
    }

    fn driver_connect(&self, in_connection_string: String) -> SqlReturn {
        const FUNCTION_NAME: &str = "driver_connect()";

        let attributes = ConnectionAttributes::parse(&in_connection_string);
        let mut arg = if let Some(dsn) = attributes.dsn() {
            read_dsn(dsn)
        } else {
            TsurugiOdbcConnectArguments::new()
        };

        if let Some(endpoint) = attributes.endpoint() {
            arg.set_endpoint(endpoint.into());
        }
        if let Some(user) = attributes.user() {
            arg.set_user(user.into());
        }
        if let Some(password) = attributes.password() {
            arg.set_password(password.into());
        }
        if let Some(token) = attributes.auth_token() {
            arg.set_auth_token(token.into());
        }
        if let Some(path) = attributes.credentials() {
            arg.set_credentials(path.into());
        }
        if let Some(credential_type) = attributes.credential_type() {
            arg.set_credential_type(credential_type);
        } else {
            #[allow(clippy::collapsible_else_if)]
            if attributes.user().is_some() {
                arg.set_credential_type(TsurugiOdbcCredentialType::UserPassword);
            } else if attributes.auth_token().is_some() {
                arg.set_credential_type(TsurugiOdbcCredentialType::AuthToken);
            } else if attributes.credentials().is_some() {
                arg.set_credential_type(TsurugiOdbcCredentialType::File);
            }
        }

        let (arg, modified) = match self.arguments_completion(arg) {
            Ok(value) => value,
            Err(rc) => return rc,
        };
        debug!("{FUNCTION_NAME}: connection_arguments={:?}", arg);

        let connection_string = to_connection_string(attributes, &arg, modified);
        let rc1 = if self.wide_char {
            write_wchar(
                "SQLDriverConnectW.out_connection_string",
                &connection_string,
                self.out_connection_string as *mut SqlWChar,
                self.out_connection_string_size,
                self.out_connection_string_length,
                Some(&self.dbc.diag_collection()),
            )
        } else {
            write_char(
                "SQLDriverConnect.out_connection_string",
                &connection_string,
                self.out_connection_string as *mut SqlChar,
                self.out_connection_string_size,
                self.out_connection_string_length,
                Some(&self.dbc.diag_collection()),
            )
        };

        let rc = connect_tsurugi(self.odbc_function_name(), &self.dbc, arg);
        rc.or(rc1)
    }

    #[cfg(windows)]
    fn arguments_completion(
        &self,
        arg: TsurugiOdbcConnectArguments,
    ) -> Result<(TsurugiOdbcConnectArguments, bool), SqlReturn> {
        const FUNCTION_NAME: &str = "arguments_completion()";

        let driver_completion = match DriverCompletion::try_from(self.driver_completion) {
            Ok(value) => value,
            Err(value) => {
                log::warn!(
                    "{FUNCTION_NAME} error. Unsupported driver_completion {:?}",
                    value
                );
                let odbc_function_name = self.odbc_function_name();
                self.dbc.add_diag(
                    TsurugiOdbcError::ConnectAttrUnsupportedDriverCompletion,
                    format!(
                        "{odbc_function_name}: Unsupported driver_completion {:?}",
                        value
                    ),
                );
                return Err(SqlReturn::SQL_ERROR);
            }
        };

        use crate::setup::dsn_dialog_windows::dsn_dialog;

        use DriverCompletion::*;
        let arg = match driver_completion {
            SQL_DRIVER_NOPROMPT => return Ok((arg, false)),
            SQL_DRIVER_PROMPT => {
                if self.hwnd.is_null() {
                    trace!("{FUNCTION_NAME} skip. hwnd is null");
                    return Ok((arg, false));
                }
                let dialog_value = DsnDialogValue::from_connect_arguments(&arg);
                match dsn_dialog(self.hwnd, dialog_value) {
                    Ok(Some(value)) => TsurugiOdbcConnectArguments::from(value),
                    Ok(None) => return Ok((arg, false)), // Cancelled by user
                    Err(_) => return Err(SqlReturn::SQL_ERROR),
                }
            }
            SQL_DRIVER_COMPLETE | SQL_DRIVER_COMPLETE_REQUIRED => {
                if arg.endpoint().is_none() {
                    if self.hwnd.is_null() {
                        trace!("{FUNCTION_NAME} skip. hwnd is null");
                        return Ok((arg, false));
                    }
                    let dialog_value = DsnDialogValue::from_connect_arguments(&arg);
                    match dsn_dialog(self.hwnd, dialog_value) {
                        Ok(Some(value)) => TsurugiOdbcConnectArguments::from(value),
                        Ok(None) => return Ok((arg, false)), // Cancelled by user
                        Err(_) => return Err(SqlReturn::SQL_ERROR),
                    }
                } else {
                    return Ok((arg, false));
                }
            }
        };

        Ok((arg, true))
    }

    #[cfg(not(windows))]
    fn arguments_completion(
        &self,
        _driver: Option<String>,
        arg: TsurugiOdbcConnectArguments,
    ) -> Result<(TsurugiOdbcConnectArguments, bool), SqlReturn> {
        // To avoid compile warnings
        let _ = self.hwnd;
        let _ = self.driver_completion;

        Ok((arg, false))
    }
}

impl From<DsnDialogValue> for TsurugiOdbcConnectArguments {
    fn from(value: DsnDialogValue) -> Self {
        let mut arg = TsurugiOdbcConnectArguments::new();
        if let Some(dsn) = value.dsn() {
            arg.set_dsn(dsn.into());
        }
        if !value.endpoint().is_empty() {
            arg.set_endpoint(value.endpoint().into());
        }

        use crate::setup::dsn_dialog_value::DsnDialogCredentialRadio::*;
        match value.credential_radio() {
            UserPassword => {
                arg.set_user(value.user().into());
                arg.set_password(value.password().into());
                arg.set_credential_type(TsurugiOdbcCredentialType::UserPassword);
            }
            AuthToken => {
                arg.set_auth_token(value.auth_token().into());
                arg.set_credential_type(TsurugiOdbcCredentialType::AuthToken);
            }
            File => {
                arg.set_credentials(value.credential_file().into());
                arg.set_credential_type(TsurugiOdbcCredentialType::File);
            }
            _ => {
                arg.set_credential_type(TsurugiOdbcCredentialType::Null);
            }
        }

        arg
    }
}

fn to_connection_string(
    attributes: ConnectionAttributes,
    arg: &TsurugiOdbcConnectArguments,
    modified: bool,
) -> String {
    let mut vec = Vec::with_capacity(8);

    if let Some(driver) = attributes.driver() {
        vec.push(format!("{}={};", KEY_DRIVER, driver));
    }
    if let Some(dsn) = attributes.dsn() {
        vec.push(format!("{}={};", KEY_DSN, dsn));
    }

    push_entry(
        &mut vec,
        KEY_ENDPOINT,
        attributes.endpoint(),
        arg.endpoint(),
        modified,
    );

    use TsurugiOdbcCredentialType::*;
    match arg.credential_type() {
        Null => {}
        UserPassword => {
            push_entry(&mut vec, KEY_USER, attributes.user(), arg.user(), modified);
            push_entry(
                &mut vec,
                KEY_PASSWORD,
                attributes.password(),
                arg.password(),
                modified,
            );
        }
        AuthToken => {
            push_entry(
                &mut vec,
                KEY_AUTH_TOKEN,
                attributes.auth_token(),
                arg.auth_token(),
                modified,
            );
        }
        File => {
            push_entry(
                &mut vec,
                KEY_CREDENTIALS,
                attributes.credentials(),
                arg.credentials(),
                modified,
            );
        }
    }

    vec.join("")
}

fn push_entry(
    vec: &mut Vec<String>,
    key: &str,
    attributes_value: Option<&String>,
    arg_value: Option<&String>,
    modified: bool,
) {
    let option_value = if modified {
        arg_value
    } else {
        attributes_value
    };
    if let Some(value) = option_value {
        vec.push(format!("{}={};", key, value));
    }
}
