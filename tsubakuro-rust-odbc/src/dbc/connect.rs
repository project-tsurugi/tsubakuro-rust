use log::{trace, warn};

use crate::{
    check_dbc,
    ctype::{SqlChar, SqlReturn, SqlSmallInt, SqlWChar},
    handle::{diag::TsurugiOdbcError, hdbc::HDbc},
    util::{char_to_string_opt, wchar_to_string_opt},
};

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

    let _server_name = char_to_string_opt(server_name, server_name_length);
    let _user_name = char_to_string_opt(user_name, user_name_length);
    let _authentication = char_to_string_opt(authentication, authentication_length);

    warn!("{dbc}.{FUNCTION_NAME} error: not yet implemented");
    dbc.add_diag(
        TsurugiOdbcError::ConnectError,
        "SQLConnect() not yet implemented",
    );
    let rc = SqlReturn::SQL_ERROR;

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

    let _server_name = wchar_to_string_opt(server_name, server_name_length);
    let _user_name = wchar_to_string_opt(user_name, user_name_length);
    let _authentication = wchar_to_string_opt(authentication, authentication_length);

    warn!("{dbc}.{FUNCTION_NAME} error: not yet implemented");
    dbc.add_diag(
        TsurugiOdbcError::ConnectError,
        "SQLConnectW() not yet implemented",
    );
    let rc = SqlReturn::SQL_ERROR;

    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc
}
