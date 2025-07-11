use std::{collections::HashMap, sync::Arc, time::Duration};

use log::{debug, trace, warn};
use tsubakuro_rust_core::prelude::*;

use crate::{
    check_dbc,
    ctype::{HWnd, SqlChar, SqlReturn, SqlSmallInt, SqlUSmallInt, SqlWChar},
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

    let map = parse_connection_string(&connection_string);
    debug!("{FUNCTION_NAME}: connection_string.map={:?}", map);

    let connection_string = map_to_connection_string(&map);
    let rc1 = write_char(
        "SQLDriverConnect.out_connection_string",
        &connection_string,
        out_connection_string,
        out_connection_string_size,
        out_connection_string_length,
        Some(&dbc.diag_collection()),
    );

    let rc = connect(&dbc, map);

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

    let map = parse_connection_string(&connection_string);
    debug!("{dbc}.{FUNCTION_NAME}: connection_string.map={:?}", map);

    let connection_string = map_to_connection_string(&map);
    let rc1 = write_wchar(
        "SQLDriverConnectW.out_connection_string",
        &connection_string,
        out_connection_string,
        out_connection_string_size,
        out_connection_string_length,
        Some(&dbc.diag_collection()),
    );

    let rc = connect(&dbc, map);

    let rc = rc.or(rc1);
    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc
}

fn parse_connection_string(s: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();

    for pair in s.split(';') {
        if pair.is_empty() {
            continue;
        }

        let mut parts = pair.splitn(2, '=');
        if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
            map.insert(key.to_string().to_lowercase(), value.to_string());
        }
    }

    map
}

fn map_to_connection_string(map: &HashMap<String, String>) -> String {
    map.iter()
        .map(|(k, v)| format!("{}={};", k, v))
        .collect::<Vec<String>>()
        .join("")
}

fn connect(dbc: &Arc<TsurugiOdbcDbc>, map: HashMap<String, String>) -> SqlReturn {
    const FUNCTION_NAME: &str = "connect()";

    if dbc.session().is_some() {
        warn!("{dbc}.{FUNCTION_NAME} error. session already exists");
        dbc.add_diag(TsurugiOdbcError::ConnectError, "session already exists");
        return SqlReturn::SQL_ERROR;
    }

    let endpoint = match map.get("endpoint") {
        Some(endpoint) => endpoint,
        None => {
            debug!("{dbc}.{FUNCTION_NAME}: endpoint not found in connection string");
            dbc.add_diag(
                TsurugiOdbcError::InvalidConnectionString,
                "endpoint not found in connection string",
            );
            return SqlReturn::SQL_ERROR;
        }
    };

    debug!("{dbc}.{FUNCTION_NAME}: endpoint={}", endpoint);
    let mut connection_option = ConnectionOption::new();
    if let Err(e) = connection_option.set_endpoint_url(endpoint) {
        debug!("{dbc}.{FUNCTION_NAME}: endpoint error. {:?}", e);
        dbc.add_diag(TsurugiOdbcError::EndpointError, e.message());
        return SqlReturn::SQL_ERROR;
    }

    let runtime = dbc.runtime();

    let timeout = Duration::from_secs(dbc.connection_timeout() as u64);
    let session = runtime.block_on(Session::connect_for(&connection_option, timeout));
    let session = match session {
        Ok(session) => {
            debug!("{dbc}.{FUNCTION_NAME}: Session::connect() succeeded");
            session
        }
        Err(e) => {
            warn!("{dbc}.{FUNCTION_NAME}: Session::connect() error. {:?}", e);
            match e {
                TgError::ClientError(message, _) => {
                    dbc.add_diag(TsurugiOdbcError::ConnectError, message);
                }
                TgError::TimeoutError(message) => {
                    dbc.add_diag(TsurugiOdbcError::ConnectTimeout, message);
                }
                TgError::IoError(message, _) => {
                    dbc.add_diag(TsurugiOdbcError::ConnectError, message);
                }
                TgError::ServerError(_, message, _, _) => {
                    dbc.add_diag(TsurugiOdbcError::ConnectError, message);
                }
            }
            return SqlReturn::SQL_ERROR;
        }
    };
    dbc.set_session(session);

    SqlReturn::SQL_SUCCESS
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_connection_string() {
        let map = parse_connection_string("Key1=Value1;key2=value2;");
        assert_eq!(Some(&"Value1".to_string()), map.get("key1"));
        assert_eq!(Some(&"value2".to_string()), map.get("key2"));
    }
}
