use std::sync::Arc;

use log::{debug, trace, warn};
use tsubakuro_rust_core::prelude::ShutdownType;

use crate::{
    check_dbc,
    ctype::SqlReturn,
    handle::{
        diag::TsurugiOdbcError,
        hdbc::{HDbc, TsurugiOdbcDbc},
    },
};

#[no_mangle]
pub extern "system" fn SQLDisconnect(hdbc: HDbc) -> SqlReturn {
    const FUNCTION_NAME: &str = "SQLDisconnect()";
    trace!("{FUNCTION_NAME} start. hdbc={:?}", hdbc);

    let dbc = check_dbc!(hdbc);

    let rc = disconnect(&dbc);

    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc
}

fn disconnect(dbc: &Arc<TsurugiOdbcDbc>) -> SqlReturn {
    const FUNCTION_NAME: &str = "disconnect()";

    let rc = if let Some(session) = dbc.clear_session() {
        let runtime = dbc.runtime();
        let rc1 = match runtime.block_on(session.shutdown(ShutdownType::Graceful)) {
            Ok(_) => {
                debug!("{dbc}.{FUNCTION_NAME}: session.shutdown() succeeded");
                SqlReturn::SQL_SUCCESS
            }
            Err(e) => {
                warn!("{dbc}.{FUNCTION_NAME}: session.shutdown() error. {:?}", e);
                let odbc_function_name = "SQLDisconnect()";
                dbc.add_diag(
                    TsurugiOdbcError::DisconnectShutdownError,
                    format!("{odbc_function_name}: shutdown error. {}", e.message()),
                );
                SqlReturn::SQL_ERROR
            }
        };
        let rc2 = match runtime.block_on(session.close()) {
            Ok(_) => {
                debug!("{dbc}.{FUNCTION_NAME}: session.close() succeeded");
                SqlReturn::SQL_SUCCESS
            }
            Err(e) => {
                warn!("{dbc}.{FUNCTION_NAME}: session.close() error. {:?}", e);
                let odbc_function_name = "SQLDisconnect()";
                dbc.add_diag(
                    TsurugiOdbcError::DisconnectCloseError,
                    format!("{odbc_function_name}: close error. {}", e.message()),
                );
                SqlReturn::SQL_ERROR
            }
        };
        rc1.or(rc2)
    } else {
        SqlReturn::SQL_SUCCESS
    };

    rc
}
