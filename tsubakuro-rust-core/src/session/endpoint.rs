use url::Url;

use crate::{client_error, error::TgError};

/// endpoint.
///
/// # Examples
/// ```
/// use tsubakuro_rust_core::prelude::*;
///
/// fn example() -> Result<ConnectionOption, TgError> {
///     let endpoint = Endpoint::parse("tcp://localhost:12345")?;
///
///     let mut connection_option = ConnectionOption::new();
///     connection_option.set_endpoint(endpoint);
///
///     Ok(connection_option)
/// }
/// ```
///
/// See [ConnectionOption::set_endpoint()](crate::prelude::ConnectionOption::set_endpoint).
#[derive(PartialEq, Clone)]
pub enum Endpoint {
    /// TCP endpoint.
    Tcp(
        /// host
        String,
        /// port
        u16,
    ),
    /// dummy.
    #[doc(hidden)]
    Other,
}

impl Endpoint {
    /// parse endpoint url.
    ///
    /// # Parameters
    /// - `endpoint` - endpoint url. (e.g. `tcp://localhost:12345`)
    pub fn parse(endpoint: &str) -> Result<Endpoint, TgError> {
        let url = Url::parse(endpoint).map_err(|e| client_error!("endpoint parse error", e))?;
        let scheme = url.scheme();
        match scheme {
            "tcp" => {
                let host = url
                    .host_str()
                    .ok_or(client_error!("tcp-endpoint parse error. host is not found"))?;
                let port = url
                    .port()
                    .ok_or(client_error!("tcp-endpoint parse error. port is not found"))?;
                Ok(Endpoint::Tcp(host.to_string(), port))
            }
            _ => Err(client_error!(format!(
                "endpoint unsupported scheme({scheme})"
            ))),
        }
    }
}

impl std::fmt::Display for Endpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Endpoint::Tcp(host, port) => write!(f, "tcp://{host}:{port}"),
            _ => panic!(),
        }
    }
}

impl std::fmt::Debug for Endpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tcp_ok_hostname() {
        let actual = Endpoint::parse("tcp://localhost:12345").unwrap();
        assert_eq!(Endpoint::Tcp("localhost".to_string(), 12345), actual);
        assert_eq!("tcp://localhost:12345", actual.to_string());
    }

    #[test]
    fn tcp_ok_ipaddress() {
        let actual = Endpoint::parse("tcp://127.0.0.1:12345").unwrap();
        assert_eq!(Endpoint::Tcp("127.0.0.1".to_string(), 12345), actual);
        assert_eq!("tcp://127.0.0.1:12345", actual.to_string());
    }

    #[test]
    fn tcp_nothing_host() {
        let _ = Endpoint::parse("tcp://:12345").unwrap_err();
    }

    #[test]
    fn tcp_nothing_host2() {
        let _ = Endpoint::parse("tcp:tsurugi").unwrap_err();
    }

    #[test]
    fn tcp_nothing_port() {
        let _ = Endpoint::parse("tcp://localhost").unwrap_err();
    }

    #[test]
    fn ng_schema() {
        let e = Endpoint::parse("ipc::tsurugi").unwrap_err();
        assert_eq!("endpoint unsupported scheme(ipc)", e.to_string());
    }
}
