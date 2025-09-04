use std::sync::Arc;

use log::{debug, trace};

use crate::{
    check_dbc,
    ctype::{SqlChar, SqlReturn, SqlSmallInt, SqlWChar},
    dbc::connect::{
        connect_tsurugi::{connect_tsurugi, TsurugiOdbcCredentialType},
        dsn::read_dsn,
    },
    handle::{
        diag::TsurugiOdbcError,
        hdbc::{HDbc, TsurugiOdbcDbc},
    },
    util::{char_to_string_opt, wchar_to_string_opt},
};

pub(crate) mod connect_tsurugi;
pub(crate) mod connection_string;
pub(crate) mod dsn;

#[no_mangle]
pub extern "system" fn SQLConnect(
    hdbc: HDbc,
    server_name: *const SqlChar,
    server_name_length: SqlSmallInt,
    user_name: *const SqlChar,
    user_name_length: SqlSmallInt,
    authentication: *const SqlChar,
    authentication_length: SqlSmallInt,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "SQLConnect()";
    trace!(
        "{FUNCTION_NAME} start. hdbc={:?}, server_name={:?}, server_name_length={:?}, user_name={:?}, user_name_length={:?}, authentication={:?}, authentication_length={:?}",
        hdbc, server_name, server_name_length, user_name, user_name_length, authentication, authentication_length
    );

    let dbc = check_dbc!(hdbc);

    let server_name = match char_to_string_opt(server_name, server_name_length) {
        Ok(value) => value,
        Err(e) => {
            debug!("{dbc}.{FUNCTION_NAME} error: {:?}", e);
            dbc.add_diag(
                TsurugiOdbcError::StringError,
                format!("{FUNCTION_NAME}: server_name convert error"),
            );
            return SqlReturn::SQL_ERROR;
        }
    };
    let user_name = char_to_string_opt(user_name, user_name_length);
    let authentication = char_to_string_opt(authentication, authentication_length);

    let rc = connect(
        FUNCTION_NAME,
        dbc,
        Ok(server_name),
        user_name,
        authentication,
        false,
    );

    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc
}

#[no_mangle]
pub extern "system" fn SQLConnectW(
    hdbc: HDbc,
    server_name: *const SqlWChar,
    server_name_length: SqlSmallInt,
    user_name: *const SqlWChar,
    user_name_length: SqlSmallInt,
    authentication: *const SqlWChar,
    authentication_length: SqlSmallInt,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "SQLConnectW()";
    trace!(
        "{FUNCTION_NAME} start. hdbc={:?}, server_name={:?}, server_name_length={:?}, user_name={:?}, user_name_length={:?}, authentication={:?}, authentication_length={:?}",
        hdbc, server_name, server_name_length, user_name, user_name_length, authentication, authentication_length
    );

    let dbc = check_dbc!(hdbc);

    let server_name = wchar_to_string_opt(server_name, server_name_length);
    let user_name = wchar_to_string_opt(user_name, user_name_length);
    let authentication = wchar_to_string_opt(authentication, authentication_length);

    let rc = connect(
        FUNCTION_NAME,
        dbc,
        server_name,
        user_name,
        authentication,
        true,
    );

    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc
}

fn connect(
    odbc_function_name: &str,
    dbc: Arc<TsurugiOdbcDbc>,
    server_name: Result<Option<String>, Box<dyn std::error::Error>>,
    user_name: Result<Option<String>, Box<dyn std::error::Error>>,
    authentication: Result<Option<String>, Box<dyn std::error::Error>>,
    _wide_char: bool,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "connect()";

    let dsn = match server_name {
        Ok(Some(value)) => value,
        Ok(None) => {
            debug!("{dbc}.{FUNCTION_NAME} error: server_name is null");
            dbc.add_diag(
                TsurugiOdbcError::StringError,
                format!("{odbc_function_name}: server_name is null"),
            );
            return SqlReturn::SQL_ERROR;
        }
        Err(e) => {
            debug!("{dbc}.{FUNCTION_NAME}: server_name convert error {:?}", e);
            dbc.add_diag(
                TsurugiOdbcError::StringError,
                format!("{odbc_function_name}: server_name convert error"),
            );
            return SqlReturn::SQL_ERROR;
        }
    };

    let user_name = match user_name {
        Ok(value) => value,
        Err(e) => {
            debug!("{dbc}.{FUNCTION_NAME}: user_name convert error {:?}", e);
            dbc.add_diag(
                TsurugiOdbcError::StringError,
                format!("{odbc_function_name}: user_name convert error"),
            );
            return SqlReturn::SQL_ERROR;
        }
    };
    let authentication = match authentication {
        Ok(value) => value,
        Err(e) => {
            debug!(
                "{dbc}.{FUNCTION_NAME}: authentication convert error {:?}",
                e
            );
            dbc.add_diag(
                TsurugiOdbcError::StringError,
                format!("{odbc_function_name}: authentication convert error"),
            );
            return SqlReturn::SQL_ERROR;
        }
    };

    let mut arg = read_dsn(&dsn);
    if let Some(user) = user_name {
        arg.set_user(user);
        arg.set_credential_type(TsurugiOdbcCredentialType::UserPassword);
    }
    if let Some(password) = authentication {
        arg.set_password(password);
    }

    connect_tsurugi(FUNCTION_NAME, &dbc, arg)
}
