use std::{collections::HashMap, sync::Arc};

use log::{debug, trace};

use crate::{
    check_dbc,
    ctype::{HWnd, SqlChar, SqlReturn, SqlSmallInt, SqlUSmallInt, SqlWChar},
    dbc::connect::{
        connect_tsurugi::{connect_tsurugi, TsurugiOdbcConnectArguments},
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

    let rc = driver_connect(FUNCTION_NAME, &dbc, map);

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

    let rc = driver_connect(FUNCTION_NAME, &dbc, map);

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

fn driver_connect(
    function_name: &str,
    dbc: &Arc<TsurugiOdbcDbc>,
    map: HashMap<String, String>,
) -> SqlReturn {
    let dsn = map.get("dsn").cloned();
    let mut arg = if let Some(dsn) = dsn {
        read_dsn(&dsn)
    } else {
        TsurugiOdbcConnectArguments::new()
    };

    if let Some(endpoint) = map.get("endpoint") {
        arg.endpoint = Some(endpoint.clone());
    }

    connect_tsurugi(function_name, dbc, arg)
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
