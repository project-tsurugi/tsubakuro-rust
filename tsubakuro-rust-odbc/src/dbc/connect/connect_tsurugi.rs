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
    pub credential: TsurugiOdbcCredential,
}

impl TsurugiOdbcConnectArguments {
    pub(crate) fn new() -> TsurugiOdbcConnectArguments {
        TsurugiOdbcConnectArguments {
            dsn: None,
            endpoint: None,
            credential: TsurugiOdbcCredential::Null,
        }
    }
}

pub(crate) enum TsurugiOdbcCredential {
    Null,
    UserPassword(String, Option<String>),
    AuthToken(String),
    File(String),
}

impl TryFrom<TsurugiOdbcCredential> for Credential {
    type Error = TgError;

    fn try_from(value: TsurugiOdbcCredential) -> Result<Self, Self::Error> {
        match value {
            TsurugiOdbcCredential::Null => Ok(Credential::null()),
            TsurugiOdbcCredential::UserPassword(user, password) => {
                Ok(Credential::from_user_password(user, password))
            }
            TsurugiOdbcCredential::AuthToken(token) => Ok(Credential::from_auth_token(token)),
            TsurugiOdbcCredential::File(path) => Ok(Credential::load(path)?),
        }
    }
}

#[derive(Debug)]
pub(crate) struct TsurugiOdbcConnectedInfo {
    endpoint: Endpoint,
    dsn: Option<String>,
}

impl TsurugiOdbcConnectedInfo {
    pub(crate) fn endpoint(&self) -> &Endpoint {
        &self.endpoint
    }

    pub(crate) fn server_name(&self) -> Option<&String> {
        match &self.endpoint {
            Endpoint::Tcp(host, _port) => Some(host),
            _ => None,
        }
    }

    pub(crate) fn dsn(&self) -> Option<&String> {
        self.dsn.as_ref()
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
    let endpoint = match Endpoint::parse(endpoint) {
        Ok(value) => value,
        Err(e) => {
            debug!("{dbc}.{FUNCTION_NAME}: endpoint parse error. {:?}", e);
            dbc.add_diag(
                TsurugiOdbcError::ConnectEndpointError,
                format!("{odbc_function_name}: endpoint parse error. {}", e),
            );
            return SqlReturn::SQL_ERROR;
        }
    };

    let credential = match Credential::try_from(arg.credential) {
        Ok(value) => value,
        Err(e) => {
            debug!("{dbc}.{FUNCTION_NAME}: credential error. {:?}", e);
            dbc.add_diag(
                TsurugiOdbcError::ConnectCredentialError,
                format!("{odbc_function_name}: credential error. {}", e),
            );
            return SqlReturn::SQL_ERROR;
        }
    };
    debug!("{dbc}.{FUNCTION_NAME}: credential={:?}", credential);

    let mut connection_option = ConnectionOption::new();
    connection_option.set_endpoint(endpoint.clone());
    connection_option.set_credential(credential);

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
                TgError::ClientError(message, cause) => match cause {
                    Some(cause) => dbc.add_diag(
                        TsurugiOdbcError::ConnectError,
                        format!("{odbc_function_name}: {} ({})", message, cause),
                    ),
                    None => {
                        dbc.add_diag(
                            TsurugiOdbcError::ConnectError,
                            format!("{odbc_function_name}: {}", message),
                        );
                    }
                },
                TgError::TimeoutError(message) => {
                    dbc.add_diag(
                        TsurugiOdbcError::ConnectTimeout,
                        format!("{odbc_function_name}: {}", message),
                    );
                }
                TgError::IoError(message, cause) => match cause {
                    Some(cause) => dbc.add_diag(
                        TsurugiOdbcError::ConnectError,
                        format!("{odbc_function_name}: {} ({})", message, cause),
                    ),
                    None => dbc.add_diag(
                        TsurugiOdbcError::ConnectError,
                        format!("{odbc_function_name}: {}", message),
                    ),
                },
                TgError::ServerError(_, message, code, server_message) => {
                    let odbc_error = if code.structured_code() == "SCD-00201" {
                        TsurugiOdbcError::ConnectAuthenticationError
                    } else {
                        TsurugiOdbcError::ConnectError
                    };
                    dbc.add_diag(
                        odbc_error,
                        format!(
                            "{odbc_function_name}: {}. {}({}): {}",
                            message,
                            code.structured_code(),
                            code.name(),
                            server_message
                        ),
                    );
                }
            }
            return SqlReturn::SQL_ERROR;
        }
    };
    dbc.set_session(session);

    let info = TsurugiOdbcConnectedInfo {
        endpoint,
        dsn: arg.dsn,
    };
    dbc.set_connected_info(info);

    SqlReturn::SQL_SUCCESS
}
