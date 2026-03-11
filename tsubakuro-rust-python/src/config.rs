use std::time::Duration;

use log::debug;
use pyo3::{prelude::*, types::*};
use pyo3_stub_gen::derive::*;
use tsubakuro_rust_core::prelude::{
    CommitOption as CoreCommitOption, ConnectionOption, Credential,
    TransactionOption as CoreTransactionOption,
};

use crate::{
    commit_option::CommitOption,
    error::{to_pyerr, InterfaceError},
    shutdown_option::ShutdownOption,
    transaction_option::TransactionOption,
};

/// Configuration options for connecting to Tsurugi.
///
/// Attributes:
///     application_name (str): Application name.
///     endpoint (str): Endpoint URL of the Tsurugi server.
///     user (str): Username for authentication.
///     password (str): Password for authentication.
///     auth_token (str): Authentication token.
///     credentials (str): Path to credentials file.
///     session_label (str): Session label for the connection.
///     transaction_option (TransactionOption): Transaction option.
///     commit_option (CommitOption): Commit option.
///     shutdown_option (ShutdownOption): Shutdown option.
///     default_timeout (int): Default timeout in seconds.
///
/// Examples:
///     ```python
///     import tsubakuro_rust_python as tsurugi
///
///     config = tsurugi.Config()
///     config.application_name = "tsubakuro-rust-python example"
///     config.endpoint = "tcp://localhost:12345"
///     config.user = "tsurugi"
///     config.password = "password"
///     config.session_label = "tsubakuro-rust-python session"
///     config.default_timeout = 30  # seconds
///     ```
///
///     ```python
///     import tsubakuro_rust_python as tsurugi
///
///     config = tsurugi.Config(
///         application_name="tsubakuro-rust-python example",
///         endpoint="tcp://localhost:12345",
///         user="tsurugi",
///         password="password",
///         session_label="tsubakuro-rust-python session",
///         default_timeout=30,  # seconds
///     )
///     ```
#[gen_stub_pyclass]
#[pyclass(module = "tsubakuro_rust_python")]
pub struct Config {
    /// Application name.
    #[pyo3(get, set)]
    application_name: Option<String>,
    /// Endpoint URL of the Tsurugi server.
    #[pyo3(get, set)]
    endpoint: Option<String>,
    /// Username for authentication.
    #[pyo3(get, set)]
    user: Option<String>,
    /// Password for authentication.
    #[pyo3(get, set)]
    password: Option<String>,
    /// Authentication token.
    #[pyo3(get, set)]
    auth_token: Option<String>,
    /// Path to credentials file.
    #[pyo3(get, set)]
    credentials: Option<String>,
    /// Session label for the connection.
    #[pyo3(get, set)]
    session_label: Option<String>,
    /// Transaction option.
    #[pyo3(get, set)]
    pub transaction_option: Option<TransactionOption>,
    /// Commit option.
    #[pyo3(get, set)]
    pub commit_option: Option<CommitOption>,
    /// Shutdown option.
    #[pyo3(get, set)]
    pub shutdown_option: Option<ShutdownOption>,
    /// Default timeout in seconds.
    #[pyo3(get, set)]
    default_timeout: Option<u64>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            application_name: None,
            endpoint: None,
            user: None,
            password: None,
            auth_token: None,
            credentials: None,
            session_label: None,
            transaction_option: None,
            commit_option: None,
            shutdown_option: None,
            default_timeout: None,
        }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl Config {
    /// Create a new `Config`.
    ///
    /// Args:
    ///     *args (Config | TransactionOption | CommitOption | ShutdownOption | str, optional): other configuration object.
    ///     **kwargs (dict, optional): e.g. `endpoint="tcp://localhost:12345"`, `user="tsurugi"`
    ///
    /// Returns:
    ///     Config: configuration object.
    #[new]
    #[pyo3(signature = (*args, **kwargs))]
    pub fn new(args: &Bound<PyTuple>, kwargs: Option<Bound<PyDict>>) -> PyResult<Self> {
        let mut slf = Self::default();
        slf.from_args(args)?;
        slf.from_kwargs(kwargs)?;
        Ok(slf)
    }

    /// Set configuration options.
    ///
    /// Args:
    ///     *args (Config | TransactionOption | CommitOption | ShutdownOption | str, optional): other configuration object.
    ///     **kwargs (dict, optional): e.g. `endpoint="tcp://localhost:12345"`, `user="tsurugi"`
    #[pyo3(signature = (*args, **kwargs))]
    pub fn set(&mut self, args: &Bound<PyTuple>, kwargs: Option<Bound<PyDict>>) -> PyResult<()> {
        self.from_args(args)?;
        self.from_kwargs(kwargs)?;
        Ok(())
    }

    /// Merge another `Config` into this one.
    ///
    /// Args:
    ///     other (Config): other configuration object.
    pub fn merge(&mut self, other: &Config) {
        if let Some(application_name) = &other.application_name {
            self.application_name = Some(application_name.clone());
        }
        if let Some(endpoint) = &other.endpoint {
            self.endpoint = Some(endpoint.clone());
        }
        if let Some(user) = &other.user {
            self.user = Some(user.clone());
        }
        if let Some(password) = &other.password {
            self.password = Some(password.clone());
        }
        if let Some(auth_token) = &other.auth_token {
            self.auth_token = Some(auth_token.clone());
        }
        if let Some(credentials) = &other.credentials {
            self.credentials = Some(credentials.clone());
        }
        if let Some(session_label) = &other.session_label {
            self.session_label = Some(session_label.clone());
        }
        if let Some(transaction_option) = &other.transaction_option {
            self.transaction_option = Some(transaction_option.clone());
        }
        if let Some(commit_option) = &other.commit_option {
            self.commit_option = Some(commit_option.clone());
        }
        if let Some(shutdown_option) = &other.shutdown_option {
            self.shutdown_option = Some(shutdown_option.clone());
        }
        if let Some(default_timeout) = &other.default_timeout {
            self.default_timeout = Some(*default_timeout);
        }
    }

    pub fn __repr__(&self) -> String {
        let none = &"None".to_string();
        let mask = &"****".to_string();
        format!(
            "Config(application_name={}, endpoint={}, user={}, password={}, auth_token={}, credentials={}, session_label={}, default_timeout={})",
            self.application_name.as_ref().unwrap_or(none),
            self.endpoint.as_ref().unwrap_or(none),
            self.user.as_ref().unwrap_or(none),
            self.password.as_ref().map_or(none, |_| mask),
            self.auth_token.as_ref().map_or(none, |_| mask),
            self.credentials.as_ref().unwrap_or(none),
            self.session_label.as_ref().unwrap_or(none),
            self.default_timeout.as_ref().map_or(none.to_string(), |v| v.to_string())
        )
    }
}

impl Config {
    pub(crate) fn from_args(&mut self, args: &Bound<PyTuple>) -> PyResult<()> {
        for item in args.iter() {
            if item.is_instance_of::<Config>() {
                let connection_option: PyRef<Config> = item.extract()?;
                self.merge(&connection_option);
                continue;
            }
            if item.is_instance_of::<TransactionOption>() {
                self.transaction_option = item.extract()?;
                continue;
            }
            if item.is_instance_of::<CommitOption>() {
                self.commit_option = item.extract()?;
                continue;
            }
            if item.is_instance_of::<ShutdownOption>() {
                self.shutdown_option = item.extract()?;
                continue;
            }

            let value: String = item.extract()?;
            for key_value in value.split_whitespace() {
                let mut parts = key_value.splitn(2, '=');
                let key = parts
                    .next()
                    .ok_or_else(|| InterfaceError::new_err("Invalid argument format"))?;
                let value = parts
                    .next()
                    .ok_or_else(|| InterfaceError::new_err("Invalid argument format"))?;
                self.set_by_string(key, value)?;
            }
        }
        Ok(())
    }

    pub(crate) fn from_kwargs(&mut self, kwargs: Option<Bound<PyDict>>) -> PyResult<()> {
        if let Some(kwargs) = kwargs {
            for (key, value) in kwargs.iter() {
                let key: String = key.extract()?;
                match key.as_str() {
                    "transaction_option" => {
                        self.transaction_option = Some(value.extract()?);
                    }
                    "commit_option" => {
                        self.commit_option = Some(value.extract()?);
                    }
                    "shutdown_option" => {
                        self.shutdown_option = Some(value.extract()?);
                    }
                    _ => {
                        self.set_by_any(&key, value)?;
                    }
                }
            }
        }
        Ok(())
    }

    fn set_by_string(&mut self, key: &str, value: &str) -> PyResult<()> {
        match key {
            "application_name" => self.application_name = Some(value.to_string()),
            "endpoint" => self.endpoint = Some(value.to_string()),
            "user" => self.user = Some(value.to_string()),
            "password" => self.password = Some(value.to_string()),
            "auth_token" => self.auth_token = Some(value.to_string()),
            "credentials" => self.credentials = Some(value.to_string()),
            "session_label" => self.session_label = Some(value.to_string()),
            "default_timeout" | "timeout" => {
                let timeout: u64 = value.parse().map_err(|_| {
                    InterfaceError::new_err("Invalid value for default_timeout/timeout")
                })?;
                self.default_timeout = Some(timeout);
            }
            _ => debug!("Unknown key: {}", key),
        }
        Ok(())
    }

    fn set_by_any(&mut self, key: &str, value: Bound<PyAny>) -> PyResult<()> {
        match key {
            "application_name" => self.application_name = Some(value.extract()?),
            "endpoint" => self.endpoint = Some(value.extract()?),
            "user" => self.user = Some(value.extract()?),
            "password" => self.password = Some(value.extract()?),
            "auth_token" => self.auth_token = Some(value.extract()?),
            "credentials" => self.credentials = Some(value.extract()?),
            "session_label" => self.session_label = Some(value.extract()?),
            "default_timeout" | "timeout" => self.default_timeout = Some(value.extract()?),
            _ => debug!("Unknown key: {}", key),
        }
        Ok(())
    }
}

impl Config {
    pub(crate) fn connection_option(&self) -> PyResult<ConnectionOption> {
        let mut connection_option = ConnectionOption::new();
        if let Some(application_name) = &self.application_name {
            connection_option.set_application_name(application_name);
        }

        if let Some(endpoint) = &self.endpoint {
            connection_option
                .set_endpoint_url(endpoint)
                .map_err(to_pyerr)?;
        } else {
            return Err(InterfaceError::new_err("endpoint is not set"));
        }

        if let Some(user) = &self.user {
            let password = self.password.as_ref();
            let credential = Credential::from_user_password(user, password);
            connection_option.set_credential(credential);
        } else if let Some(auth_token) = &self.auth_token {
            let credential = Credential::from_auth_token(auth_token);
            connection_option.set_credential(credential);
        } else if let Some(credentials) = &self.credentials {
            let credential = Credential::load(credentials).map_err(to_pyerr)?;
            connection_option.set_credential(credential);
        }

        if let Some(session_label) = &self.session_label {
            connection_option.set_session_label(session_label);
        }

        connection_option.set_default_timeout(self.default_timeout());

        Ok(connection_option)
    }

    pub(crate) fn connect_timeout(&self) -> Duration {
        self.default_timeout()
    }

    pub(crate) fn core_transaction_option(&self) -> CoreTransactionOption {
        if let Some(transaction_option) = &self.transaction_option {
            transaction_option.to_core_transaction_option()
        } else {
            TransactionOption::default().to_core_transaction_option()
        }
    }

    pub(crate) fn core_commit_option(&self) -> CoreCommitOption {
        if let Some(commit_option) = &self.commit_option {
            commit_option.to_core_commit_option()
        } else {
            CommitOption::default().to_core_commit_option()
        }
    }

    pub(crate) fn shutdown_option(&self) -> Option<&ShutdownOption> {
        self.shutdown_option.as_ref()
    }

    pub(crate) fn default_timeout(&self) -> Duration {
        if let Some(timeout) = self.default_timeout {
            Duration::from_secs(timeout)
        } else {
            Duration::ZERO
        }
    }
}
