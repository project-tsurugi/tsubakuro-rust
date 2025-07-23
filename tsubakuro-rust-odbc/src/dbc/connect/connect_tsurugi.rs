use std::{sync::Arc, time::Duration};

use log::{debug, warn};
use tsubakuro_rust_core::prelude::*;

use crate::{
    ctype::SqlReturn,
    handle::{diag::TsurugiOdbcError, hdbc::TsurugiOdbcDbc},
};

pub(crate) struct TsurugiOdbcConnectArguments {
    pub dsn: Option<String>,
    pub endpoint: Option<String>,
    pub user_name: Option<String>,
    pub authentication: Option<String>,
}

impl TsurugiOdbcConnectArguments {
    pub(crate) fn new() -> TsurugiOdbcConnectArguments {
        TsurugiOdbcConnectArguments {
            dsn: None,
            endpoint: None,
            user_name: None,
            authentication: None,
        }
    }
}

pub(crate) fn connect_tsurugi(
    odbc_function_name: &str,
    dbc: &Arc<TsurugiOdbcDbc>,
    arg: TsurugiOdbcConnectArguments,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "connect_tsurugi()";

    if dbc.session().is_some() {
        warn!("{dbc}.{FUNCTION_NAME} error. session already exists");
        dbc.add_diag(
            TsurugiOdbcError::ConnectError,
            format!("{odbc_function_name}: session already exists"),
        );
        return SqlReturn::SQL_ERROR;
    }

    let endpoint = match arg.endpoint {
        Some(ref value) => value,
        None => {
            debug!("{dbc}.{FUNCTION_NAME} error. endpoint not found");
            dbc.add_diag(
                TsurugiOdbcError::ConnectEndpointNotFound,
                format!("{odbc_function_name}: endpoint not found"),
            );
            return SqlReturn::SQL_ERROR;
        }
    };

    debug!("{dbc}.{FUNCTION_NAME}: endpoint={}", endpoint);
    let mut connection_option = ConnectionOption::new();
    if let Err(e) = connection_option.set_endpoint_url(endpoint) {
        debug!("{dbc}.{FUNCTION_NAME}: endpoint error. {:?}", e);
        dbc.add_diag(
            TsurugiOdbcError::ConnectEndpointError,
            format!("{odbc_function_name}: endpoint error. {}", e.message()),
        );
        return SqlReturn::SQL_ERROR;
    }

    let runtime = dbc.runtime();

    let timeout = Duration::from_secs(dbc.connection_timeout());
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
                    dbc.add_diag(
                        TsurugiOdbcError::ConnectError,
                        format!("{odbc_function_name}: {}", message),
                    );
                }
                TgError::TimeoutError(message) => {
                    dbc.add_diag(
                        TsurugiOdbcError::ConnectTimeout,
                        format!("{odbc_function_name}: {}", message),
                    );
                }
                TgError::IoError(message, _) => {
                    dbc.add_diag(
                        TsurugiOdbcError::ConnectError,
                        format!("{odbc_function_name}: {}", message),
                    );
                }
                TgError::ServerError(_, message, _, _) => {
                    dbc.add_diag(
                        TsurugiOdbcError::ConnectError,
                        format!("{odbc_function_name}: {}", message),
                    );
                }
            }
            return SqlReturn::SQL_ERROR;
        }
    };
    dbc.set_session(session);

    SqlReturn::SQL_SUCCESS
}
