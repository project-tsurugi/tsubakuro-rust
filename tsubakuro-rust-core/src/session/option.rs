use std::time::Duration;

use crate::error::TgError;

use super::endpoint::Endpoint;

/// connection option.
#[derive(Debug, Clone)]
pub struct ConnectionOption {
    endpoint: Option<Endpoint>,
    application_name: Option<String>,
    session_label: Option<String>,
    keep_alive: Duration,
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
    pub fn new() -> ConnectionOption {
        ConnectionOption {
            endpoint: None,
            application_name: None,
            session_label: None,
            keep_alive: Duration::from_secs(60),
            default_timeout: Duration::ZERO,
            send_timeout: Duration::ZERO,
            recv_timeout: Duration::ZERO,
        }
    }

    pub fn set_endpoint(&mut self, endpoint: Endpoint) {
        self.endpoint = Some(endpoint);
    }

    pub fn set_endpoint_url(&mut self, endpoint: &str) -> Result<(), TgError> {
        let endpoint = Endpoint::parse(endpoint)?;
        self.set_endpoint(endpoint);
        Ok(())
    }

    pub fn endpoint(&self) -> Option<&Endpoint> {
        self.endpoint.as_ref()
    }

    pub fn set_application_name(&mut self, name: &str) {
        self.application_name = Some(name.to_string());
    }

    pub fn application_name(&self) -> Option<&String> {
        self.application_name.as_ref()
    }

    pub fn set_session_label(&mut self, label: &str) {
        self.session_label = Some(label.to_string());
    }

    pub fn session_label(&self) -> Option<&String> {
        self.session_label.as_ref()
    }

    // ZEROのときはキープアライブしない
    pub fn set_keep_alive(&mut self, keep_alive: Duration) {
        self.keep_alive = keep_alive;
    }

    pub fn keep_alive(&self) -> Duration {
        self.keep_alive
    }

    pub fn set_default_timeout(&mut self, timeout: Duration) {
        self.default_timeout = timeout;
    }

    pub fn default_timeout(&self) -> Duration {
        self.default_timeout
    }

    pub fn set_send_timeout(&mut self, timeout: Duration) {
        self.send_timeout = timeout;
    }

    pub fn send_timeout(&self) -> Duration {
        self.send_timeout
    }

    pub fn set_recv_timeout(&mut self, timeout: Duration) {
        self.recv_timeout = timeout;
    }

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
