use std::{sync::Arc, time::Duration};

use log::{debug, warn};
use tsubakuro_rust_core::prelude::*;

use crate::{
    ctype::SqlReturn,
    handle::{diag::TsurugiOdbcError, hdbc::TsurugiOdbcDbc},
};

#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub(crate) enum TsurugiOdbcCredentialType {
    Null = 1,
    UserPassword = 2,
    AuthToken = 3,
    File = 4,
}

impl TryFrom<&str> for TsurugiOdbcCredentialType {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use TsurugiOdbcCredentialType::*;
        match value {
            "1" => Ok(Null),
            "2" => Ok(UserPassword),
            "3" => Ok(AuthToken),
            "4" => Ok(File),
            _ => Err(()),
        }
    }
}

#[derive(Clone)]
pub(crate) struct TsurugiOdbcConnectArguments {
    dsn: Option<String>,
    endpoint: Option<String>,
    user: Option<String>,
    password: Option<String>,
    auth_token: Option<String>,
    credentials: Option<String>,
    credential_type: TsurugiOdbcCredentialType,
}

impl std::fmt::Debug for TsurugiOdbcConnectArguments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsurugiOdbcConnectArguments")
            .field("dsn", &self.dsn)
            .field("endpoint", &self.endpoint)
            .field("user", &self.user)
            .field("password", &"****")
            .field("auth_token", &"****")
            .field("credentials", &self.credentials)
            .field("credential_type", &self.credential_type)
            .finish()
    }
}

impl TsurugiOdbcConnectArguments {
    pub(crate) fn new() -> TsurugiOdbcConnectArguments {
        TsurugiOdbcConnectArguments {
            dsn: None,
            endpoint: None,
            user: None,
            password: None,
            auth_token: None,
            credentials: None,
            credential_type: TsurugiOdbcCredentialType::Null,
        }
    }

    pub fn set_dsn(&mut self, dsn: String) {
        self.dsn = Some(dsn);
    }

    pub fn dsn(&self) -> Option<&String> {
        self.dsn.as_ref()
    }

    pub fn set_endpoint(&mut self, endpoint: String) {
        self.endpoint = Some(endpoint);
    }

    pub fn endpoint(&self) -> Option<&String> {
        self.endpoint.as_ref()
    }

    pub fn set_user(&mut self, user: String) {
        self.user = Some(user);
    }

    pub fn user(&self) -> Option<&String> {
        self.user.as_ref()
    }

    pub fn set_password(&mut self, password: String) {
        self.password = Some(password);
    }

    pub fn password(&self) -> Option<&String> {
        self.password.as_ref()
    }

    pub fn set_auth_token(&mut self, auth_token: String) {
        self.auth_token = Some(auth_token);
    }

    pub fn auth_token(&self) -> Option<&String> {
        self.auth_token.as_ref()
    }

    pub fn set_credentials(&mut self, path: String) {
        self.credentials = Some(path);
    }

    pub fn credentials(&self) -> Option<&String> {
        self.credentials.as_ref()
    }

    pub fn set_credential_type(&mut self, credential_type: TsurugiOdbcCredentialType) {
        self.credential_type = credential_type;
    }

    pub fn credential_type(&self) -> TsurugiOdbcCredentialType {
        self.credential_type
    }

    #[allow(clippy::result_large_err)]
    fn credential(&self) -> Result<Credential, TgError> {
        use TsurugiOdbcCredentialType::*;
        match self.credential_type {
            Null => Ok(Credential::Null),
            UserPassword => Ok(Credential::from_user_password(
                self.user.as_ref().unwrap_or(&"".into()),
                self.password.as_ref(),
            )),
            AuthToken => Ok(Credential::from_auth_token(
                self.auth_token.as_ref().unwrap_or(&"".into()),
            )),
            File => Ok(Credential::load(
                self.credentials.as_ref().unwrap_or(&"".into()),
            )?),
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

    let credential = match arg.credential() {
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
