use std::collections::HashMap;

const DSN: &str = "DSN";
pub(crate) const ENDPOINT: &str = "Endpoint";

#[derive(Debug)]
pub(crate) struct ConnectionAttributes {
    attributes: HashMap<String, String>,
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
                attributes.set(key.to_string(), value.to_string());
            }
        }

        attributes
    }

    pub fn set(&mut self, key: String, value: String) {
        self.attributes.insert(key.to_lowercase(), value);
    }

    pub(crate) fn get(&self, key: &str) -> Option<&String> {
        self.attributes.get(&key.to_lowercase())
    }

    pub fn dsn(&self) -> Option<&String> {
        self.get(DSN)
    }

    pub fn endpoint(&self) -> Option<&String> {
        self.get(ENDPOINT)
    }
}

impl std::fmt::Display for ConnectionAttributes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pairs: Vec<String> = self
            .attributes
            .iter()
            .map(|(k, v)| format!("{}={};", k, v))
            .collect();
        write!(f, "{}", pairs.join(""))
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

        let s = attributes.to_string();
        if s.starts_with("key1") {
            assert_eq!("key1=Value1;key2=value2;", s);
        } else {
            assert_eq!("key2=value2;key1=Value1;", s);
        }
    }
}
