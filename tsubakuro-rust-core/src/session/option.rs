use std::{path::Path, time::Duration};

use crate::{
    error::TgError,
    prelude::{
        r#type::large_object::{LargeObjectRecvPathMapping, LargeObjectSendPathMapping},
        Credential,
    },
};

use super::endpoint::Endpoint;

/// The default validity period for UserPasswordCredential in seconds.
const DEFAULT_VALIDITY_PERIOD_SECONDS: u64 = 300;

/// Option to connect to Tsurugi server.
///
/// See [Session::connect()](crate::prelude::Session::connect).
///
/// # Examples
/// ```
/// use std::sync::Arc;
/// use tsubakuro_rust_core::prelude::*;
///
/// async fn example() -> Result<Arc<Session>, TgError> {
///     let credential = Credential::from_user_password("user", Some("password"));
///
///     let mut connection_option = ConnectionOption::new();
///     connection_option.set_endpoint_url("tcp://localhost:12345")?;
///     connection_option.set_credential(credential);
///     connection_option.set_application_name("Tsubakuro/Rust example");
///     connection_option.set_session_label("example session");
///     connection_option.set_default_timeout(std::time::Duration::from_secs(10));
///
///     let session = Session::connect(&connection_option).await?;
///
///     Ok(session)
/// }
/// ```
#[derive(Debug, Clone)]
pub struct ConnectionOption {
    endpoint: Option<Endpoint>,
    credential: Credential,
    validity_period: Duration,
    application_name: Option<String>,
    session_label: Option<String>,
    keep_alive: Duration,
    lob_send_path_mapping: LargeObjectSendPathMapping,
    lob_recv_path_mapping: LargeObjectRecvPathMapping,
    default_timeout: Duration,
    send_timeout: Duration,
    recv_timeout: Duration,
}

impl Default for ConnectionOption {
    fn default() -> Self {
        Self::new()
    }
}

impl ConnectionOption {
    /// Creates a new instance.
    pub fn new() -> ConnectionOption {
        ConnectionOption {
            endpoint: None,
            credential: Credential::Null,
            validity_period: Duration::from_secs(DEFAULT_VALIDITY_PERIOD_SECONDS),
            application_name: None,
            session_label: None,
            keep_alive: Duration::from_secs(60),
            lob_send_path_mapping: LargeObjectSendPathMapping::new(),
            lob_recv_path_mapping: LargeObjectRecvPathMapping::new(),
            default_timeout: Duration::ZERO,
            send_timeout: Duration::ZERO,
            recv_timeout: Duration::ZERO,
        }
    }

    /// Set endpoint.
    pub fn set_endpoint(&mut self, endpoint: Endpoint) {
        self.endpoint = Some(endpoint);
    }

    /// Set endpoint.
    ///
    /// # Parameters
    /// - `endpoint` - endpoint url. (e.g. `tcp://localhost:12345`)
    pub fn set_endpoint_url(&mut self, endpoint: &str) -> Result<(), TgError> {
        let endpoint = Endpoint::parse(endpoint)?;
        self.set_endpoint(endpoint);
        Ok(())
    }

    /// Get endpoint.
    pub fn endpoint(&self) -> Option<&Endpoint> {
        self.endpoint.as_ref()
    }

    /// Set credential.
    ///
    /// since 0.5.0
    pub fn set_credential(&mut self, credential: Credential) {
        self.credential = credential;
    }

    /// Get credential.
    ///
    /// since 0.5.0
    pub fn credential(&self) -> &Credential {
        &self.credential
    }

    /// Set validity period for UserPasswordCredential.
    ///
    /// For internal use.
    ///
    /// since 0.5.0
    pub fn set_validity_period(&mut self, duration: Duration) {
        self.validity_period = duration;
    }

    /// Get validity period.
    ///
    /// For internal use.
    ///
    /// since 0.5.0
    pub fn validity_period(&self) -> Duration {
        self.validity_period
    }

    /// Set application name.
    pub fn set_application_name(&mut self, name: &str) {
        self.application_name = Some(name.to_string());
    }

    /// Get application name.
    pub fn application_name(&self) -> Option<&String> {
        self.application_name.as_ref()
    }

    /// Set session label.
    pub fn set_session_label(&mut self, label: &str) {
        self.session_label = Some(label.to_string());
    }

    /// Get session label
    pub fn session_label(&self) -> Option<&String> {
        self.session_label.as_ref()
    }

    /// Set keep alive interval.
    ///
    /// Do not keep alive when `keep_alive` is 0.
    pub fn set_keep_alive(&mut self, keep_alive: Duration) {
        self.keep_alive = keep_alive;
    }

    /// Get keep alive interval.
    pub fn keep_alive(&self) -> Duration {
        self.keep_alive
    }

    /// Adds a path mapping entry for both sending and receiving BLOB/CLOB.
    ///
    /// since 0.2.0
    pub fn add_large_object_path_mapping<T: AsRef<Path>>(
        &mut self,
        client_path: T,
        server_path: &str,
    ) {
        self.add_large_object_path_mapping_on_send(&client_path, server_path);
        self.add_large_object_path_mapping_on_recv(server_path, &client_path);
    }

    /// Adds a path mapping entry for sending BLOB/CLOB.
    ///
    /// since 0.2.0
    pub fn add_large_object_path_mapping_on_send<T: AsRef<Path>>(
        &mut self,
        client_path: T,
        server_path: &str,
    ) {
        self.lob_send_path_mapping.add(client_path, server_path);
    }

    /// Adds a path mapping entry for receiving BLOB/CLOB.
    ///
    /// since 0.2.0
    pub fn add_large_object_path_mapping_on_recv<T: AsRef<Path>>(
        &mut self,
        server_path: &str,
        client_path: T,
    ) {
        self.lob_recv_path_mapping.add(server_path, client_path);
    }

    pub(crate) fn large_object_path_mapping_on_send(&self) -> &LargeObjectSendPathMapping {
        &self.lob_send_path_mapping
    }

    pub(crate) fn large_object_path_mapping_on_recv(&self) -> &LargeObjectRecvPathMapping {
        &self.lob_recv_path_mapping
    }

    /// Set default timeout.
    pub fn set_default_timeout(&mut self, timeout: Duration) {
        self.default_timeout = timeout;
    }

    /// Get default timeout.
    pub fn default_timeout(&self) -> Duration {
        self.default_timeout
    }

    /// Set communication send timeout.
    pub fn set_send_timeout(&mut self, timeout: Duration) {
        self.send_timeout = timeout;
    }

    /// Get communication send timeout.
    pub fn send_timeout(&self) -> Duration {
        self.send_timeout
    }

    /// Set communication recv timeout.
    pub fn set_recv_timeout(&mut self, timeout: Duration) {
        self.recv_timeout = timeout;
    }

    /// Get communication recv timeout.
    pub fn recv_timeout(&self) -> Duration {
        self.recv_timeout
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn endpoint() {
        let mut option = ConnectionOption::new();

        let endpoint: Endpoint = Endpoint::parse("tcp://localhost:12345").unwrap();
        option.set_endpoint(endpoint);

        assert_eq!(
            Some(&Endpoint::Tcp("localhost".to_string(), 12345)),
            option.endpoint()
        );
    }

    #[test]
    fn endpoint_url_str() {
        let mut option = ConnectionOption::new();

        let endpoint: &str = "tcp://localhost:12345";
        option.set_endpoint_url(endpoint).unwrap();

        assert_eq!(
            Some(&Endpoint::Tcp("localhost".to_string(), 12345)),
            option.endpoint()
        );
    }

    #[test]
    fn endpoint_url_string() {
        let mut option = ConnectionOption::new();

        let endpoint: String = "tcp://localhost:12345".to_string();
        option.set_endpoint_url(&endpoint).unwrap();

        assert_eq!(
            Some(&Endpoint::Tcp("localhost".to_string(), 12345)),
            option.endpoint()
        );
    }

    #[test]
    fn application_name_str() {
        let mut option = ConnectionOption::new();

        let name: &str = "appname-test";
        option.set_application_name(name);

        assert_eq!(Some(&name.to_string()), option.application_name());
    }

    #[test]
    fn application_name_string() {
        let mut option = ConnectionOption::new();

        let name: String = "appname-test".to_string();
        option.set_application_name(&name);

        assert_eq!(Some(&name), option.application_name());
    }

    #[test]
    fn label_str() {
        let mut option = ConnectionOption::new();

        let label: &str = "label-test";
        option.set_session_label(label);

        assert_eq!(Some(&label.to_string()), option.session_label());
    }

    #[test]
    fn label_string() {
        let mut option = ConnectionOption::new();

        let label: String = "label-test".to_string();
        option.set_session_label(&label);

        assert_eq!(Some(&label), option.session_label());
    }

    #[test]
    fn default_timeout() {
        let mut option = ConnectionOption::new();
        assert_eq!(Duration::ZERO, option.default_timeout());

        option.set_default_timeout(Duration::from_secs(123));
        assert_eq!(Duration::from_secs(123), option.default_timeout());
    }

    #[test]
    fn send_timeout() {
        let mut option = ConnectionOption::new();
        assert_eq!(Duration::ZERO, option.send_timeout());

        option.set_send_timeout(Duration::from_secs(123));
        assert_eq!(Duration::from_secs(123), option.send_timeout());
    }

    #[test]
    fn recv_timeout() {
        let mut option = ConnectionOption::new();
        assert_eq!(Duration::ZERO, option.recv_timeout());

        option.set_recv_timeout(Duration::from_secs(123));
        assert_eq!(Duration::from_secs(123), option.recv_timeout());
    }
}
