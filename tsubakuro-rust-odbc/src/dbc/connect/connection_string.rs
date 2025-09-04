use std::collections::HashMap;

use crate::dbc::connect::connect_tsurugi::TsurugiOdbcCredentialType;

pub(crate) const KEY_DRIVER: &str = "Driver";
pub(crate) const KEY_DSN: &str = "DSN";
pub(crate) const KEY_ENDPOINT: &str = "Endpoint";
pub(crate) const KEY_USER: &str = "UID";
pub(crate) const KEY_PASSWORD: &str = "PWD";
pub(crate) const KEY_AUTH_TOKEN: &str = "AuthToken";
pub(crate) const KEY_CREDENTIALS: &str = "Credentials";
pub(crate) const KEY_CREDENTIAL_TYPE: &str = "CredentialType";

pub(crate) struct ConnectionAttributes {
    attributes: HashMap<String, String>,
}

impl std::fmt::Debug for ConnectionAttributes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pairs: Vec<String> = self
            .attributes
            .iter()
            .map(|(k, v)| {
                if *k == KEY_PASSWORD.to_lowercase() || *k == KEY_AUTH_TOKEN.to_lowercase() {
                    format!("{}=****", k)
                } else {
                    format!("{}={}", k, v)
                }
            })
            .collect();
        write!(f, "ConnectionAttributes {{ {} }}", pairs.join(", "))
    }
}

impl ConnectionAttributes {
    pub fn new() -> Self {
        Self {
            attributes: HashMap::new(),
        }
    }

    pub fn parse(connection_string: &str) -> Self {
        let mut attributes = ConnectionAttributes::new();

        for pair in connection_string.split(';') {
            if pair.is_empty() {
                continue;
            }

            let mut parts = pair.splitn(2, '=');
            if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
                attributes.set(key, value.to_string());
            }
        }

        attributes
    }

    pub fn set(&mut self, key: &str, value: String) {
        self.attributes.insert(key.to_lowercase(), value);
    }

    pub(crate) fn get(&self, key: &str) -> Option<&String> {
        self.attributes.get(&key.to_lowercase())
    }

    pub fn driver(&self) -> Option<&String> {
        self.get(KEY_DRIVER)
    }

    pub fn dsn(&self) -> Option<&String> {
        self.get(KEY_DSN)
    }

    pub fn endpoint(&self) -> Option<&String> {
        self.get(KEY_ENDPOINT)
    }

    pub fn user(&self) -> Option<&String> {
        self.get(KEY_USER)
    }

    pub fn password(&self) -> Option<&String> {
        self.get(KEY_PASSWORD)
    }

    pub fn auth_token(&self) -> Option<&String> {
        self.get(KEY_AUTH_TOKEN)
    }

    pub fn credentials(&self) -> Option<&String> {
        self.get(KEY_CREDENTIALS)
    }

    pub fn credential_type(&self) -> Option<TsurugiOdbcCredentialType> {
        match self.get(KEY_CREDENTIAL_TYPE) {
            Some(value) => TsurugiOdbcCredentialType::try_from(value.as_str()).ok(),
            None => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_connection_string() {
        let attributes = ConnectionAttributes::parse("Key1=Value1;key2=value2;");
        assert_eq!(Some(&"Value1".to_string()), attributes.get("key1"));
        assert_eq!(Some(&"value2".to_string()), attributes.get("key2"));
    }
}
