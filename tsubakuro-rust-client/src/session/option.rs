use crate::error::TgError;

use super::endpoint::Endpoint;

/// connection option.
#[derive(Debug, Clone)]
pub struct ConnectionOption {
    endpoint: Option<Endpoint>,
    application_name: Option<String>,
    label: Option<String>,
}

impl ConnectionOption {
    pub fn new() -> ConnectionOption {
        ConnectionOption {
            endpoint: None,
            application_name: None,
            label: None,
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

    pub fn set_label(&mut self, label: &str) {
        self.label = Some(label.to_string());
    }

    pub fn label(&self) -> Option<&String> {
        self.label.as_ref()
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
        option.set_label(label);

        assert_eq!(Some(&label.to_string()), option.label());
    }

    #[test]
    fn label_string() {
        let mut option = ConnectionOption::new();

        let label: String = "label-test".to_string();
        option.set_label(&label);

        assert_eq!(Some(&label), option.label());
    }
}
