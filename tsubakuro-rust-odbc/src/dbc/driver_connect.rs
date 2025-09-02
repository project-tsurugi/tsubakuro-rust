use std::sync::Arc;

use log::{debug, trace};

use crate::{
    check_dbc,
    ctype::{HWnd, SqlChar, SqlReturn, SqlSmallInt, SqlUSmallInt, SqlWChar},
    dbc::connect::{
        connect_tsurugi::{connect_tsurugi, TsurugiOdbcConnectArguments, TsurugiOdbcCredential},
        connection_string::ConnectionAttributes,
        dsn::read_dsn,
    },
    handle::{
        diag::TsurugiOdbcError,
        hdbc::{HDbc, TsurugiOdbcDbc},
    },
    util::{char_to_string, wchar_to_string, write_char, write_wchar},
};

#[no_mangle]
pub extern "system" fn SQLDriverConnect(
    hdbc: HDbc,
    _window_handle: HWnd,
    in_connection_string: *const SqlChar,
    in_connection_string_length: SqlSmallInt,
    out_connection_string: *mut SqlChar,
    out_connection_string_size: SqlSmallInt,
    out_connection_string_length: *mut SqlSmallInt,
    driver_completion: SqlUSmallInt,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "SQLDriverConnect()";
    trace!(
        "{FUNCTION_NAME} start. hdbc={:?}, in_connection_string={:?}, in_connection_string_length={:?}, out_connection_string={:?}, out_connection_string_size={:?}, out_connection_string_length={:?}, driver_completion={:?}",
        hdbc, in_connection_string, in_connection_string_length,out_connection_string, out_connection_string_size, out_connection_string_length, driver_completion
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

    let attributes = ConnectionAttributes::parse(&connection_string);
    debug!("{FUNCTION_NAME}: connection_attributes={:?}", attributes);

    let connection_string = attributes.to_string();
    let rc1 = write_char(
        "SQLDriverConnect.out_connection_string",
        &connection_string,
        out_connection_string,
        out_connection_string_size,
        out_connection_string_length,
        Some(&dbc.diag_collection()),
    );

    let rc = driver_connect(FUNCTION_NAME, &dbc, attributes);

    let rc = rc.or(rc1);
    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc
}

#[no_mangle]
pub extern "system" fn SQLDriverConnectW(
    hdbc: HDbc,
    _window_handle: HWnd,
    in_connection_string: *const SqlWChar,
    in_connection_string_length: SqlSmallInt,
    out_connection_string: *mut SqlWChar,
    out_connection_string_size: SqlSmallInt,
    out_connection_string_length: *mut SqlSmallInt,
    driver_completion: SqlUSmallInt,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "SQLDriverConnectW()";
    trace!(
        "{FUNCTION_NAME} start. hdbc={:?}, in_connection_string={:?}, in_connection_string_length={:?}, out_connection_string={:?}, out_connection_string_size={:?}, out_connection_string_length={:?}, driver_completion={:?}",
        hdbc, in_connection_string, in_connection_string_length,out_connection_string, out_connection_string_size, out_connection_string_length, driver_completion
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

    let attributes = ConnectionAttributes::parse(&connection_string);
    debug!("{FUNCTION_NAME}: connection_attributes={:?}", attributes);

    let connection_string = attributes.to_string();
    let rc1 = write_wchar(
        "SQLDriverConnectW.out_connection_string",
        &connection_string,
        out_connection_string,
        out_connection_string_size,
        out_connection_string_length,
        Some(&dbc.diag_collection()),
    );

    let rc = driver_connect(FUNCTION_NAME, &dbc, attributes);

    let rc = rc.or(rc1);
    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc
}

fn driver_connect(
    function_name: &str,
    dbc: &Arc<TsurugiOdbcDbc>,
    attributes: ConnectionAttributes,
) -> SqlReturn {
    let mut arg = if let Some(dsn) = attributes.dsn() {
        read_dsn(dsn)
    } else {
        TsurugiOdbcConnectArguments::new()
    };

    if let Some(endpoint) = attributes.endpoint() {
        arg.endpoint = Some(endpoint.clone());
    }
    set_credential(&mut arg, &attributes);

    connect_tsurugi(function_name, dbc, arg)
}

fn set_credential(arg: &mut TsurugiOdbcConnectArguments, attributes: &ConnectionAttributes) {
    if let Some(user) = attributes.user() {
        let password = attributes.password();
        arg.credential = TsurugiOdbcCredential::UserPassword(user.into(), password.cloned());
        return;
    }

    if let Some(token) = attributes.auth_token() {
        arg.credential = TsurugiOdbcCredential::AuthToken(token.into());
        return;
    }

    if let Some(path) = attributes.credentials() {
        arg.credential = TsurugiOdbcCredential::File(path.into());
        return;
    }
}
