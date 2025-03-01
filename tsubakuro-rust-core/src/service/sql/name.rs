use crate::jogasaki::proto::sql::response::Name as ProtoName;

/// Table name.
#[derive(Clone, PartialEq)]
pub struct TName {
    identifiers: Vec<String>,
}

impl TName {
    /// Creates a new instance.
    pub fn new(identifiers: Vec<String>) -> TName {
        TName { identifiers }
    }

    pub(crate) fn from(proto_name: &ProtoName) -> TName {
        let identifiers = proto_name
            .identifiers
            .iter()
            .map(|identifier| identifier.label.to_string())
            .collect();
        TName { identifiers }
    }

    /// Get identifiers.
    pub fn identifiers(&self) -> &Vec<String> {
        &self.identifiers
    }
}

impl std::fmt::Display for TName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.identifiers.join(".");
        write!(f, "{}", s)
    }
}

impl std::fmt::Debug for TName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tname_identifiers() {
        let name = TName::new(vec![
            String::from("scheme1"),
            String::from("database1"),
            String::from("table1"),
        ]);

        let i = name.identifiers();
        assert_eq!(3, i.len());
        assert_eq!(String::from("scheme1"), i[0]);
        assert_eq!(String::from("database1"), i[1]);
        assert_eq!(String::from("table1"), i[2]);
    }

    #[test]
    fn tname_to_string1() {
        let name = TName::new(vec![String::from("table1")]);
        assert_eq!("table1", name.to_string());
    }

    #[test]
    fn tname_to_string3() {
        let name = TName::new(vec![
            String::from("scheme1"),
            String::from("database1"),
            String::from("table1"),
        ]);
        assert_eq!("scheme1.database1.table1", name.to_string());
    }
}
